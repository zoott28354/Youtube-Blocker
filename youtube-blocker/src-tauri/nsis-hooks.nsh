; YouTube Blocker — NSIS installer hooks
; Questi macro vengono inclusi nel template NSIS di Tauri via `installerHooks`.
; L'hook PREUNINSTALL chiede all'utente se vuole ripristinare il sistema
; (hosts, firewall, policy browser) prima della disinstallazione.

!macro NSIS_HOOK_PREINSTALL
!macroend

!macro NSIS_HOOK_POSTINSTALL
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  ; Chiedi all'utente se vuole ripristinare le impostazioni di sistema.
  ; L'app viene chiamata con --cleanup prima che i file vengano cancellati.
  MessageBox MB_YESNO|MB_ICONQUESTION \
    "Ripristinare le impostazioni di sistema prima di disinstallare?$\r$\n$\r$\nVerranno rimossi:$\r$\n  - Voci nel file hosts (blocco DNS)$\r$\n  - Regole firewall anti-DoH$\r$\n  - Policy browser (Chrome, Edge, Brave, Firefox)$\r$\n$\r$\nRecommended: Yes" \
    IDNO ytb_cleanup_skip
    ExecWait '"$INSTDIR\youtube-blocker.exe" --cleanup'
  ytb_cleanup_skip:
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
!macroend
