# Naviguer vers votre dossier AppData actuel
$appDataPath = "$env:APPDATA\com.devopsbenjamin.hypnovr"

# Créer le ZIP
Compress-Archive -Path "$appDataPath\playlists", "$appDataPath\sessions", "$appDataPath\songs", "$appDataPath\assets"`
                 -DestinationPath "./ressource-pack.zip" `
                 -Force

Write-Host "RessourcePack data ZIP created successfully!"