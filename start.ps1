param (
    [string]$env = "dev"
)

# Définir les fichiers docker-compose en fonction de l'environnement
switch ($env) {
    "prod" {
        $composeFiles = "docker-compose.yml", "docker-compose.prod.yml"
    }
    "dev" {
        $composeFiles = "docker-compose.yml", "docker-compose.dev.yml"
    }
    default {
        Write-Error "Environnement non reconnu. Utilisez 'prod' ou 'dev'."
        exit 1
    }
}


# Construire l'argument -f pour docker-compose
$composeArgs = $composeFiles | ForEach-Object { "-f $_" }

# Exécuter la commande docker-compose
docker-compose @composeArgs up --build
