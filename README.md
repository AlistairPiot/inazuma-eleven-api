# API Inazuma Eleven

Une API simple écrite en **Rust** avec **Actix Web**, qui fournit des informations sur les joueurs d'Inazuma Eleven. Les données sont stockées dans un fichier `data.json` et l’API les expose via HTTP en JSON.

## Fonctionnalités

-   Récupérer la liste complète des joueurs.
-   Chaque joueur contient :
    -   Nom (`name`)
    -   Rareté (`rarity`)
    -   Poste (`position`)
    -   Élément (`element`)
    -   Nom de l’équipe (`team`)

## Installation

1. Cloner le dépôt :

```bash
git clone <URL_DE_TON_DEPOT>
cd nom_du_projet
```
