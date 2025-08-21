## Finance/Trading

## Summary
Describe the main change (feature, bug fix, docs, performance, config).


- Type of change (pick one):
- [ ] New feature
- [ ] Bug fix
- [ ] Non‑breaking change
- [ ] Breaking change (requires major version bump)
- [ ] Documentation
- [ ] Performance / Optimization
- [ ] Infrastructure / CI


## Background & Motivation
*What problem does this solve?* Provide context, references (papers, links), and related issues/feature requests (#ID).


## Technical Details
- Affected modules/components: `...`
- Trading algorithm/strategy: `...` (e.g., mean reversion, momentum, stat‑arb, market making, pairs)
- Risk model changes: `...` (VaR, max drawdown, Kelly, position sizing)
- Data & sources: `...` (exchange, vendor, frequency, license)
- Backtest: parameters, period, fees/slippage, OOS/Walk‑Forward mode
- Performance: before vs after (CAGR, Sharpe, Sortino, MaxDD, WinRate, Calmar)


## Backtest/Benchmark Results
_Include a table/summary + charts (attach artifacts or upload via CI)._


| Metric | Before | After |
|---|---:|---:|
| CAGR | | |
| Sharpe | | |
| Sortino | | |
| Max Drawdown | | |
| Hit Ratio | | |
| Turnover | | |


> **Note**: State transaction costs, slippage, latency, execution mode (backtest vs paper/live), and the random seed.


## Breaking/Compatibility Impact
- [ ] Public interface changed
- [ ] Data/schema migration required
- [ ] Computation results changed (due to bugfix/seed/params)
- [ ] None


## Quality Checklist
- [ ] Unit tests pass & coverage > **X%**
- [ ] Lint/format (ruff/black/eslint/prettier) pass
- [ ] Docs updated (README/API/CHANGELOG)
- [ ] Look‑ahead/peeking avoided
- [ ] Data leakage tested (train/val/test split)
- [ ] Parameters tracked (config/MLflow/W&B)
- [ ] Random seed set
- [ ] Transaction costs & slippage modeled
- [ ] Sensitivity & robustness checks (grid/bootstrapping)
- [ ] Data rights & licenses verified


## Risk & Compliance
- Potential impact to capital/operational risk: `...`
- Leverage/exposure limits: `...`
- Compliance: none / data license / broker API TOS / LEGAL REVIEW NEEDED


## Screenshots/Charts
_Insert equity curve, return distribution, drawdown, exposure, confusion matrix (if ML), etc._


## Anything Else to Review
`...`

---
name: Bug report
about: Help us fix a problem
labels: bug
---


## Bug Description
What happened vs what you expected.


## Steps to Reproduce
1. ...
2. ...


## Logs/Traceback

### b) `feature_request.md`
```md
---
name: Feature request
about: Propose an idea or enhancement
labels: enhancement
---


## Problem/Gaps
What user/research problem are we solving?


## Proposed Solution
Feature overview, API, or architectural changes.


## Alternatives Considered
Other options and trade‑offs.


## Technical Impact
- Related modules:
- Schema/data changes:
- Estimated effort:


## Acceptance Criteria
- [ ] ...
- [ ] ...

# Contributing Guidelines – Finance/Trading


Thanks for contributing! This document standardizes research & engineering quality so results are reproducible and bias‑aware.


## Workflow
1. **Fork** the repo, create a feature branch: `feat/feature-name` or fix branch: `fix/bug-name`.
2. Ensure local linting & tests pass.
3. Update relevant documentation.
4. Open a **Pull Request** using our template; include backtest results and artifacts.


## Code Standards
- Python: `ruff` + `black`; Node/TS: `eslint` + `prettier`.
- Strong typing (mypy/pyright/tsc) for strategy/risk modules.
- All public functions must have docstrings (Google/Numpy style).


## Data & Licensing
- Respect exchange/vendor TOS. Do not commit closed/proprietary datasets.
- Clearly declare data source, timestamp, timezone, and license.


## Sound Backtesting Practices
- Separate **in‑sample** vs **out‑of‑sample**; use **walk‑forward** when appropriate.
- Model **transaction costs, slippage, latency**, and **liquidity** constraints.
- Avoid **look‑ahead bias, survivorship bias**, and data leakage.
- Report metrics: CAGR, Sharpe, Sortino, Calmar, MaxDD, Turnover, Exposure.
- Provide **seed** and **config** for reproducibility.


## Testing
- Unit tests for signals, sizing, risk, broker integration.
- Golden tests for data pipelines.
- Minimum performance thresholds for strategies to be mergeable.


## Security
- Do not store API keys in the repo. Use **dotenv**/secrets manager.
- Follow `SECURITY.md` for vulnerability reporting.


## Communication
- Use issues/discussions for design. Include architecture diagrams for complex changes.

# Security Policy


If you discover a vulnerability, **do not** open a public issue.


- Email: security@example.repo
- Target response time: 72 hours
- We support **responsible disclosure**.

# Code of Conduct


We are committed to an inclusive community. Be respectful, avoid harassment, and follow GitHub Community Guidelines. Violations may result in warnings or bans.

# Format: path @owner1 @owner2
* @core-team @lead-quant
/docs/* @tech-writer
/strategies @lead-quant @qa

## Contributing
See [CONTRIBUTING.md](./CONTRIBUTING.md) for coding, data, and backtesting standards. Open PRs using our template and include backtest results.

ci-python.yml
name: CI (Python)


on:
pull_request:
branches: [ main ]
push:
branches: [ main ]


jobs:
build:
runs-on: ubuntu-latest
steps:
- uses: actions/checkout@v4
- uses: actions/setup-python@v5
with:
python-version: '3.11'
- name: Cache pip
uses: actions/cache@v4
with:
path: ~/.cache/pip
key: ${{ runner.os }}-pip-${{ hashFiles('**/requirements*.txt') }}
- name: Install deps
run: |
pip install -r requirements.txt
pip install ruff black mypy pytest
- name: Lint
run: |
ruff check .
black --check .
mypy .
- name: Test
run: pytest -q
- name: Upload artifacts (backtest results)
if: always()
uses: actions/upload-artifact@v4
with:
name: backtest-results
path: artifacts/**

name: CI (Node)


on:
pull_request:
branches: [ main ]


jobs:
build:
runs-on: ubuntu-latest
steps:
- uses: actions/checkout@v4
- uses: actions/setup-node@v4
with:
node-version: '20'
- name: Install
run: npm ci
- name: Lint & Test
run: |
npm run lint
npm test --silent

**PR Title**: [feat] Momentum Strategy v2 with Risk‑Parity Sizing


### Summary
Implements a daily multi‑asset momentum strategy with risk‑parity position sizing. Adds `strategies/momentum_v2.py`, parameterization via `config/momentum_v2.yaml`, and a backtest pipeline.


### Background
Momentum v1 suffered from high drawdowns in volatile regimes. v2 introduces a volatility filter and target‑volatility allocation.


### Technical Details
- Signals: ROC(63) + MA crossover(50/200) + volatility filter (ATR 20)
- Sizing: risk parity via inverse vol, max 20% per asset
- Risk: ATR‑based stops, portfolio target vol 10%
- Data: Binance spot (BTC, ETH) + ETFs (SPY, TLT) via vendor X, daily, UTC
- Costs: 10 bps/trade; slippage: 5 bps


### Backtest Results (2018‑01‑01 — 2025‑06‑30)
| Metric | v1 | v2 |
|---|---:|---:|
| CAGR | 12.1% | 17.4% |
| Sharpe | 0.92 | 1.28 |
| Sortino | 1.20 | 1.73 |
| MaxDD | -34.5% | -22.3% |
| Turnover | 3.1 | 2.4 |


_Charts & notebooks attached as artifacts._


### Impact & Compatibility
- Changes default config; no breaking changes to the public API.


### Checklist
- [x] Unit + integration tests pass
- [x] Linting passes
- [x] Docs updated: README, docstrings, CHANGELOG
- [x] Repro: seed, config, fees, slippage stated
- [x] Data licenses verified

# Changelog


## [Unreleased]
- ...


## [1.1.0] - 2025-08-22
### Added
- Momentum strategy v2; risk‑parity sizing.


### Fixed
- Slippage calculation in broker simulator.


### Changed
- Default transaction cost to 10 bps.

.github/
ISSUE_TEMPLATE/
bug_report.md
feature_request.md
workflows/
ci-python.yml
ci-node.yml
strategies/
momentum_v2.py
config/
momentum_v2.yaml
artifacts/
(backtest outputs & charts)
PULL_REQUEST_TEMPLATE.md
CONTRIBUTING.md
CODE_OF_CONDUCT.md
CODEOWNERS
SECURITY.md
CHANGELOG.md
DISCLAIMER.md
README.md

