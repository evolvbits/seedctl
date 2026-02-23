pub const WARNING_TEXT: &str = r#"
***************************************************************
  EXTREME SECURITY MODE — COLD WALLET ENVIRONMENT REQUIRED
***************************************************************

  You are about to reveal the MASTER KEYS to your funds.

  If ANYONE gains access to this data, your assets can be stolen
  instantly and irreversibly.

  DO NOT CONTINUE unless ALL conditions below are true:

  • This machine is permanently OFFLINE (Wi-Fi, Ethernet, Bluetooth
    disabled or physically removed)
  • You booted from a clean, trusted OS (live system recommended)
  • No cameras, microphones, or screen recording devices are present
  • No one else can see your screen
  • This device has NEVER been infected or used for daily browsing
  • You understand that malware can steal secrets invisibly
  • You accept full responsibility for key exposure

  STRICT RULES:

  • NEVER store your mnemonic phrase, passphrase, and word indexes digitally and/or share them
  • NEVER paste it into any website
  • NEVER photograph it
  • NEVER upload it
  • NEVER reuse it on a hot wallet

  LOSS WARNING:

  If you lose your mnemonic or passphrase, your funds are PERMANENTLY
  LOST.
  No support. No recovery. No exceptions.

  OPERATION PROTOCOL:

  1. Generate wallet
  2. Write mnemonic + passphrase on paper (temporary)
  3. Transfer to a permanent offline backup (steel plate recommended)
  4. Verify backup accuracy
  5. DESTROY temporary paper copy
  6. CLOSE this program immediately"#;
