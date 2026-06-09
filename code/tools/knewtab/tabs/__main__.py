from tabs.boilerplate import Config, google_sheet as sheet, update_manifest_version
from tabs.paths import *

# https://feathericons.com

z = Config()

# ==============================================================================

z["p.mail"] = ("mail.proton.me", "mail")
z["outlook"] = ("outlook.office365.com/mail", "mail")
z["g.mail"] = ("gmail.com", "mail")
z["g.drive"] = ("drive.google.com", "hard-drive")

z["calendar"] = ("calendar.google.com", "calendar")
z["job.app"] = (sheet("12kmFEuOpieg_fpT5aCUyXpqC954iZ7MbVtZ9Sb0GDFw"), "table")
z["grad.app"] = ("gradapp.nus.edu.sg/apply", "send")
z["notes"] = ("app.notesnook.com", "file-text")

# l("lean api", "leanprover-community.github.io/mathlib4_docs", i="book")
z["ibkr"] = ("interactivebrokers.com.sg/sso/Login?RL=1", "bar-chart-2")
z["dbs"] = ("internet-banking.dbs.com.sg", "globe")
z["emoji"] = ("emoji.julien-marcou.fr", "image")
z["resume"] = ("github.com/nguyenvukhang/hire", "github")

# ==============================================================================

z.save(OUTPUT_HTML)


with open(VERSION_FILE, "r") as f:
    version = f.read().strip()
    update_manifest_version(version)
