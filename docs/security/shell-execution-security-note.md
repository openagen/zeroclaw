# Note de sécurité — Outil `shell` (exécution de commandes)

**Statut :** Comportement actuel (pas une proposition)
**Date :** 2026-02-22
**Périmètre :** `src/tools/shell.rs`, `src/security/policy.rs`

---

## 1. Surface concernée

L'outil `shell` permet à l'agent d'exécuter des commandes arbitraires dans le
répertoire de travail configuré. C'est la surface la plus sensible du runtime :
une exécution de commande peut modifier le système de fichiers, exfiltrer des
données, établir des connexions réseau ou élever des privilèges.

---

## 2. Défenses en place

### 2.1 Politique d'autonomie (`AutonomyLevel`)

| Niveau | Comportement shell |
|---|---|
| `ReadOnly` | Toutes les commandes bloquées |
| `Supervised` | Exécution après validation par `validate_command_execution` |
| `Autonomous` | Exécution après validation, sans approbation humaine requise |

La vérification est faite avant toute exécution — il n'y a pas de chemin de
contournement.

### 2.2 Validation de commande (`validate_command_execution`)

Les commandes à risque moyen ou élevé requièrent que le paramètre `approved: true`
soit explicitement fourni par l'appelant. Ce mécanisme protège contre une exécution
autonome non souhaitée de commandes destructives.

### 2.3 Rate limiting

Deux niveaux de protection par débit :

- `is_rate_limited()` — vérifié **avant** la validation de commande.
- `record_action()` — décrémente le budget d'actions par heure ; vérifié juste avant l'exécution.

Le budget `max_actions_per_hour = 0` bloque complètement l'outil.

### 2.4 Isolation de l'environnement (`env_clear` + `SAFE_ENV_VARS`)

Le processus enfant démarre avec un environnement **vidé** (`env_clear()`).
Seules les variables suivantes sont réinjectées depuis l'environnement parent :

```
PATH, HOME, TERM, LANG, LC_ALL, LC_CTYPE, USER, SHELL, TMPDIR
```

Aucune variable contenant `KEY`, `SECRET`, ou `TOKEN` n'est dans cette liste.
Un test (`shell_does_not_leak_api_key`) valide cette propriété à chaque CI.

### 2.5 Timeout (60 secondes)

Toute commande dépassant 60 secondes est terminée. Cela protège contre les
processus suspendus indéfiniment (boucles infinies, attentes réseau, fork bombs).

### 2.6 Troncature de sortie (1 Mo)

La sortie stdout et stderr est tronquée à 1 Mo chacune. Cela prévient les
allocations mémoire excessives sur les commandes produisant un volume élevé.

---

## 3. Risques résiduels et gaps connus

### 3.1 Pas d'isolation du système de fichiers par défaut

**Niveau de risque :** Moyen

La commande s'exécute dans le répertoire de travail configuré, mais rien
n'empêche une commande de remonter l'arborescence (`../../etc/passwd`) ou
d'écrire hors du workspace si l'utilisateur dispose des droits.

**Atténuation disponible :** Landlock (activé automatiquement sur Linux 5.13+ depuis
ce sprint) restreint l'accès au workspace, `/tmp`, `/usr`, et `/bin`. Sur les
noyaux antérieurs ou non-Linux, le runtime repasse sur Firejail (si disponible)
ou sur l'application-layer seul.

### 3.2 Pas de filtrage de commandes réseau

**Niveau de risque :** Moyen

`curl`, `wget`, `nc`, `ssh` et d'autres outils réseau sont accessibles si présents
dans le `PATH`. Une commande approuvée peut exfiltrer des données ou ouvrir des
connexions sortantes arbitraires.

**Recommandation opérateur :** Combiner avec une politique d'egress réseau (firewall,
iptables rules) sur l'hôte. Landlock ne contrôle pas les sockets réseau.

### 3.3 Héritage des ressources du processus parent

**Niveau de risque :** Faible

Le processus enfant hérite des descripteurs de fichiers ouverts du parent (sauf
marqués `O_CLOEXEC`). Cela peut exposer des sockets ou des fichiers ouverts au
sous-processus si le runtime ne les ferme pas.

**Statut :** Risque accepté, délégué à `tokio::process::Command` qui ferme les fds
non-hérités par défaut sur les plateformes supportées.

### 3.4 Injection via le contenu de la commande

**Niveau de risque :** Faible à moyen

Le paramètre `command` est passé tel quel à un shell (`sh -c`). Si l'agent
construit dynamiquement une commande en interpolant des données utilisateur sans
sanitisation, une injection shell est possible.

**Défense en place :** Le paramètre `approved` requiert une approbation explicite
pour les commandes à risque. La validation de commande peut rejeter des patterns
dangereux via la politique de sécurité. La chaîne de construction de la commande
(`build_shell_command`) est sous la responsabilité du `RuntimeAdapter`.

**Recommandation :** Le modèle de prompt doit être configuré pour ne pas construire
de commandes en interpolant directement des données non fiables.

---

## 4. Matrice de couverture des tests

| Scénario | Test |
|---|---|
| Commande autorisée exécutée | `shell_executes_allowed_command` |
| Commande bloquée (politique) | `shell_blocks_disallowed_command` |
| Mode ReadOnly bloque tout | `shell_blocks_readonly` |
| Paramètre `command` manquant | `shell_missing_command_param` |
| Type incorrect pour `command` | `shell_wrong_type_param` |
| Exit code non-zéro capturé | `shell_captures_exit_code` |
| API_KEY non transmis au shell | `shell_does_not_leak_api_key` |
| PATH et HOME disponibles | `shell_preserves_path_and_home` |
| Approbation requise (risque moyen) | `shell_requires_approval_for_medium_risk_command` |
| Rate limit bloque l'exécution | `shell_blocks_rate_limited` |
| Constante timeout = 60s | `shell_timeout_constant_is_reasonable` |
| Limite sortie = 1 Mo | `shell_output_limit_is_1mb` |
| SAFE_ENV_VARS exclut les secrets | `shell_safe_env_vars_excludes_secrets` |
| SAFE_ENV_VARS inclut les essentiels | `shell_safe_env_vars_includes_essentials` |

---

## 5. Recommandations opérateur

```toml
[security]
# Restreindre le niveau d'autonomie en production
autonomy = "supervised"

# Budget d'actions par heure (0 = désactivé)
max_actions_per_hour = 50

# Répertoire de travail isolé — éviter de pointer vers /
workspace_dir = "/var/zeroclaw/workspace"
```

Sur Linux, Landlock est activé automatiquement si le noyau est ≥ 5.13. Vérifier
avec `dmesg | grep landlock` ou consulter les logs de démarrage de l'agent.

---

## 6. Rollback

Ce document décrit le comportement actuel. En cas de modification de `shell.rs`
ou de `SecurityPolicy`, mettre à jour ce document en même temps que le PR.
