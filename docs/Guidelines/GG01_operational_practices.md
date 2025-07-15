# Operational Practices

This guideline describes standard operational procedures for KAIRO deployments.


## 3. Automation Interface Priority

### 3.1. Browser Automation with 'Automate'
- **Constraint:** The current version of the 'Automate' tool identifies browser windows by their tab title string.
- **Problem:** The 'Commander GPT' interface changes its tab title with each conversation, causing identification failures.
- **Resolution:** For any automated workflow involving browser tab manipulation via 'Automate', the 'Co-commander Gemini' interface, which maintains a static tab title ('Google Gemini'), shall be used as the primary target to ensure process stability.
