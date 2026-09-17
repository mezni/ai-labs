# Omni-Connect Phase 2 — Representative Copilot MVP (Months 3–5)

Phase 2 focuses on building, testing, and deploying the AI Sales Copilot MVP
directly into the hands of retail store representatives across 10–15 pilot
stores. The primary objective is to drive **Employee Productivity** by
eliminating multi-app context switching, providing real-time customer
insights, and guiding reps toward faster transaction completions and higher
sales conversion rates.

---

## 1. High-Level Objectives

- **Single-Pane-of-Glass Interface:** Deploy an intuitive Copilot interface
  that aggregates Customer 360, usage data, and promotional eligibility into
  a single screen.
- **Reduce Handling Time:** Cut initial Average Handling Time (AHT) from
  35–50 minutes down to 20–25 minutes during the pilot.
- **Accelerate Representative Onboarding:** Reduce new employee training time
  to 1–2 weeks via guided conversational workflows and automated next-best
  actions.
- **Drive Adoption & Feedback:** Achieve 80%+ daily active rep adoption in
  pilot stores while establishing real-time feedback loops to refine prompts
  and recommendations.

---

## 2. Core Workstreams & Deliverables

```
                 ┌──────────────────────────────────────────────┐
                 │           PHASE 2 WORKSTREAMS                │
                 └────────────────────┬─────────────────────────┘
                                      │
     ┌───────────────────┬────────────┴─────────────┬───────────────────┐
     ▼                   ▼                         ▼                   ▼
┌──────────┐      ┌────────────┐            ┌───────────────┐    ┌──────────┐
│ Workstream│      │ Workstream │            │  Workstream   │    │ Workstream│
│    1      │      │     2      │            │       3       │    │    4      │
│  UI/UX & │      │  Core AI & │            │ Real-time Data│    │ Field Pilot│
│  Rep Exp │      │ Prompt Eng │            │  Integrations │    │ & Operations│
└──────────┘      └────────────┘            └───────────────┘    └──────────┘
```

### Workstream 1: Representative Interface & UX Design

**Detailed Task:** Design an embedded sidecar or standalone web/tablet UI
that integrates cleanly into existing POS/CRM screens without requiring full
system rewrites.

- **Deliverable 1.1:** Copilot Interface Wireframes & Design System optimized
  for rapid scanning during live customer interactions.
- **Deliverable 1.2:** One-Click Action Triggers (e.g., auto-filling plan
  codes, launching upgrade checks).

### Workstream 2: Core AI Engine & Knowledge RAG

**Detailed Task:** Configure the conversational AI engine, prompt templates,
and Retrieval-Augmented Generation (RAG) pipeline over company SOPs, rate
plans, and promotion rules.

- **Deliverable 2.1:** Context Assembly Engine that dynamically constructs LLM
  prompts using live Customer 360 payloads.
- **Deliverable 2.2:** Knowledge Base Vector Index enabling sub-second search
  across battlecards, plan terms, and troubleshooting guides.

### Workstream 3: Real-Time Integration & Orchestration

**Detailed Task:** Connect the API layer built in Phase 1 to the Copilot
backend, enabling live data fetching for device inventory, credit eligibility,
and active promotions.

- **Deliverable 3.1:** Middleware Orchestration Layer handling real-time API
  calls to legacy backends with fallback caching.
- **Deliverable 3.2:** Audit & Telemetry Service logging all queries, generated
  recommendations, and rep acceptance rates for performance tracking.

### Workstream 4: Pilot Deployment & Store Change Management

**Detailed Task:** Roll out the Copilot MVP to 10–15 pilot stores, conduct rep
training, and monitor operational telemetry.

- **Deliverable 4.1:** 1-Week Representative Onboarding Program & Training
  Modules.
- **Deliverable 4.2:** Pilot Performance Dashboard tracking AHT, lookup times,
  conversion rates, and rep feedback.

---

## 3. Phase 2 Execution Steps (Playbook)

### Step 1 — Build UI Components & Prompt Templates (Weeks 1–3, Month 3)

- Develop the front-end sidecar/tablet widget featuring customer summary
  cards, recommended plans, and a conversational query box.
- Engineer prompt templates for core rep queries ("What is the best plan for
  this customer?", "Is this line eligible for a trade-in promotion?").
- Connect the RAG pipeline to retrieve promotional rules and competitive
  battlecards instantly.

### Step 2 — Integrate APIs & Establish Context Assembly (Weeks 4–6, Months 3–4)

- Connect the Copilot interface to the Customer 360 API aggregator (billing,
  usage, line status).
- Implement prompt-guardrail middleware for real-time PII redaction and
  response formatting.
- Perform end-to-end load and latency testing to ensure total response
  generation takes **under 1.5 seconds**.

### Step 3 — Pilot Store Deployment & Rep Onboarding (Weeks 7–9, Month 4)

- Deploy the Copilot MVP to the 10–15 designated pilot stores.
- Conduct intensive 2-day training sessions for pilot store representatives
  and store managers.
- Activate in-app feedback widgets (thumbs up/down, brief comment box) for
  every AI response.

### Step 4 — Iterate, Optimize & Measure Impact (Weeks 10–12, Month 5)

- Analyze daily usage logs, telemetry, and feedback to refine system prompts
  and RAG retrieval accuracy.
- Compare pilot store metrics against control stores (AHT, conversion,
  onboarding speed).
- Conduct the **Phase 2 Executive Gate Review** to validate success criteria
  and approve Phase 3 funding.

---

## 4. Phase 2 Key Success Criteria (Gate Review Metrics)

To complete Phase 2 and transition into Phase 3 (Customer AI Assistant &
Omnichannel Engagement), the MVP must achieve the following pilot metrics:

| Metric / Objective | Baseline | Phase 2 MVP Target | Measurement Method |
|---|---|---|---|
| Representative Adoption | 0% | ≥80% Daily Active Usage | System Audit Logs |
| Average Handling Time (AHT) | 35–50 min | 20–25 min | POS Session Telemetry |
| Information Lookup Time | 3–5 min across 5-10 apps | <30 seconds via Copilot | UI Telemetry & Timers |
| Sales Conversion Rate | Baseline | +10–15% increase | POS Transaction Data vs. Control Stores |
| Employee Onboarding Time | 4–6 weeks | 1–2 weeks | HR & Field Operations Tracking |

---

## 5. Business Understanding: AI Representative Copilot (Phase 2 MVP)

```
┌───────────────────────────────────────────────────────────────┐
│                     BUSINESS DECISION FRAMEWORK                │
├──────────────────────────┬────────────────────────────────────┤
│ Core Decision            │ Deploy AI Sales Copilot across      │
│                          │ Pilot Stores                       │
├──────────────────────────┼────────────────────────────────────┤
│ Primary Driver           │ Reduce AHT (35-50m → 20-25m) &      │
│                          │ Multi-App Friction                 │
├──────────────────────────┼────────────────────────────────────┤
│ System Governance        │ Human-in-the-Loop                  │
│                          │ (Rep Retains Final Sign-off)       │
└──────────────────────────┴────────────────────────────────────┘
```

### 5.1 Core Business Decision

**Primary Decision:** Deploy an AI Sales Copilot sidecar interface to retail
representatives across 10–15 pilot stores to act as the primary
recommendation, lookup, and action engine during live customer interactions.

**Scope of Autonomous Authority — Human-in-the-Loop (Assisted Execution):**
The AI Copilot generates insights, retrieves promotion eligibility, and
recommends next-best plans, but the representative must explicitly review and
confirm any transaction before committing changes to core billing or
provisioning backends.

**Operational Boundary:** Covers in-store account reviews, device upgrades,
rate plan changes, and promotional matching. Excludes complex credit risk
overrides, manual fraud clears, and hardware trade-in exception approvals
(which still require store manager sign-off).

### 5.2 Key Constraints

**Technical & Performance Constraints**

- **Retrieval & Generation Latency:** Aggregate end-to-end response time must
  remain under 1.5 seconds (<500 ms backend API fetch + <1.0 s LLM inference)
  to prevent adding delays to live customer conversations.
- **Zero Legacy Overhaul:** Must integrate into existing tablet/POS hardware
  via lightweight UI overlay/sidecar without replacing legacy billing
  (Amdocs/Netcracker) or CRM backends.

**Regulatory, Security & Compliance Constraints**

- **CPNI & PII Compliance:** All Customer Proprietary Network Information and
  personally identifiable information must be masked or redacted before
  hitting public/LLM processing layers.
- **Zero Hallucination Tolerance on Pricing:** Promotional terms, trade-in
  valuations, and monthly plan rates generated by RAG must be **100%
  deterministic** and backed by verified catalog IDs.

**Operational & Adoption Constraints**

- **Minimal Training Overhead:** Rep interface must require no more than 2
  hours of formal training, supporting intuitive use by new hires within
  their first week.
- **System Reliability:** 99.9% uptime during retail operational hours
  (8 AM – 9 PM across local store time zones).

### 5.3 Ownership Structure & Accountability

| Domain | Responsible Role / Owner | Primary Accountability |
|---|---|---|
| Business & Value Realization | VP, Retail Operations & Field Transformation | P&L impact, store adoption rates, AHT reduction, rep experience |
| Product & Features | Lead AI Product Manager (Retail Digital) | Feature backlog, prompt engineering quality, UI/UX workflow, rep feedback loops |
| Technical Architecture & Data | Enterprise Architect / Head of AI Engineering | Real-time API connections, Customer 360 data pipelines, RAG accuracy, system latency |
| Governance & Security | Chief Information Security Officer (CISO) & Legal Counsel | CPNI/PII data masking, regulatory compliance, access controls, security sign-offs |
| Field Execution | Pilot Store Managers & Regional Retail Directors | Day-to-day rep compliance, pilot feedback collection, change management in stores |

### 5.4 Success Criteria & Gating Metrics

The Phase 2 MVP will be judged against five explicit operational, financial,
and technical gates to secure executive approval and funding for Phase 3:

```
                  PHASE 2 MVP SUCCESS GATING METRICS
 ┌─────────────────────────────────────────────────────────┐
 │  [Adopt]  80%+ Rep Daily Active Usage in Pilot Stores   │
 │  [Speed]  AHT reduced from 35-50 min to 20-25 min        │
 │  [Lookup] Information retrieval time reduced to <30 sec  │
 │  [Sales]  +10% to +15% boost in Sales Conversion Rate    │
 │  [Speed]  Employee Onboarding time cut to 1-2 weeks      │
 └─────────────────────────────────────────────────────────┘
```

- **Representative Adoption:** ≥80% Daily Active Usage (DAU) among active reps
  in the 10–15 pilot stores.
- **Average Handling Time (AHT):** Reduction from baseline 35–50 minutes down
  to 20–25 minutes (a ~40–50% drop).
- **Information Retrieval Speed:** Information lookup time reduced from 3–5
  minutes (across 5–10 apps) to <30 seconds via unified search.
- **Sales Impact:** +10% to +15% increase in upgrade and plan sales
  conversion rates compared to control stores.
- **Onboarding Efficiency:** Time required for new hires to reach full
  operational proficiency reduced from 4–6 weeks to 1–2 weeks.

> Focusing tightly on a Phase 2 MVP is the smartest move — it delivers
> immediate operational relief to reps, proves ROI to executive sponsors, and
> builds the technical foundation for later phases without taking on
> unnecessary scope creep.

---

## 6. MVP Scope & Capabilities (The "Must-Haves")

To keep the MVP lightweight, fast, and high-impact, Phase 2 focuses
exclusively on three core rep workflows where context-switching causes the
most friction:

```
┌──────────────────────────────────────────────────────────────────┐
│                    PHASE 2 MVP CORE CAPABILITIES                 │
├──────────────────────────────┬───────────────────────────────────┤
│ 1. Unified Customer 360 Card │ Aggregates account status, usage, │
│                              │ device info                      │
├──────────────────────────────┼───────────────────────────────────┤
│ 2. Automated Eligibility     │ Instant trade-in, promo, and plan │
│    Check                     │ upgrade rules                    │
├──────────────────────────────┼───────────────────────────────────┤
│ 3. Conversational RAG        │ Sub-second natural language lookup│
│    Copilot                   │ for SOPs / Promos                │
└──────────────────────────────┴───────────────────────────────────┘
```

**In Scope for MVP:**

- Single-pane customer account summary (tenure, current plan, device financing
  balance, monthly data usage).
- One-click plan recommendation engine based on 3-month data/voice usage.
- Instant promo & upgrade eligibility matching (calculates device trade-in
  value & stackable discounts).
- RAG-powered search across rate plans, terms, and competitive battlecards.

**Out of Scope for MVP (Deferred to Phase 3/4):**

- Automated payment processing or credit overrides.
- Direct customer self-service or kiosk integrations.
- Hardware diagnostics or automated eSIM provisioning.

---

## 7. MVP User Experience (The Sidecar Overlay)

Instead of rebuilding the existing Point-of-Sale (POS) or CRM, the MVP
deploys as a smart sidecar widget (web or tablet app) that sits alongside
current store software.

```
┌──────────────────────────────┬──────────────────────────────────┐
│ EXISTING POS / CRM SCREEN    │ AI COPILOT SIDECAR (MVP)         │
│                              │                                  │
│ Customer: John Doe           │ 👤 JOHN DOE (Tenure: 4 yrs)       │
│ Account #: 987654321         │ 📱 Current: iPhone 13 (Fin $0)    │
│ Status: Active               │ 📊 Avg Data: 42 GB/mo (Unlim St)  │
│                              │ ───────────────────────────────  │
│ [ Standard POS Form Fields. ]│ 💡 RECOMMENDED NEXT BEST ACTION: │
│                              │ Upgrade to Unlimited + iPhone 16 │
│                              │ • Qualified for $800 Trade-in    │
│                              │ • Monthly bill change: +$5/mo    │
│                              │ [ Apply Recommendation to POS ]  │
│                              │ ───────────────────────────────  │
│                              │ 💬 Ask Copilot...                │
│                              │ (e.g. "What is the trade-in      │
│                              │  policy?")                       │
└──────────────────────────────┴──────────────────────────────────┘
```

---

## 8. Step-by-Step Implementation Plan (12-Week Sprint Cycle)

### Step 1 — Weeks 1–3: Data Integration & API Wire-Up (Data Backbone)

- Connect the sidecar UI to the 4 core APIs: Customer Profile,
  Billing/Usage, Device Catalog, and Promo Engine.
- Load promotional rules and SOP documents into the vector database (RAG
  layer).
- Implement PII data masking middleware (anonymize names and account numbers
  before sending queries to the LLM).

### Step 2 — Weeks 4–6: Prompt Engineering & UI Development (Rep Experience)

- Build the responsive sidecar interface (React / web widget optimized for
  store tablets and desktops).
- Fine-tune system prompt templates for Next-Best Action recommendations and
  side-by-side plan comparisons.
- Conduct latency optimization to keep total query-to-response generation
  under 1.5 seconds.

### Step 3 — Weeks 7–9: Pilot Deployment & Store Training (Go-Live)

- Deploy the MVP to 10–15 pilot stores (training 50–75 representatives).
- Conduct a 2-hour hands-on training module covering how to use the sidecar
  during live customer interactions.
- Enable real-time feedback buttons (👍/👎 and brief text boxes) directly on
  every AI response.

### Step 4 — Weeks 10–12: Telemetry Tracking & Executive Gate Review

- Measure baseline vs. pilot store performance: AHT reduction, lookup times,
  and conversion rates.
- Adjust prompt instructions based on rep feedback logs (e.g., simplifying
  recommendation summaries).
- Present pilot results to the Executive Steering Committee to unlock Phase 3
  funding.

---

## 9. MVP Success Criteria & Gating Metrics

To declare Phase 2 a success and receive sign-off for Phase 3 (Customer AI
Assistant), the MVP must achieve the following target gates in pilot stores:

| Metric | Pre-AI Baseline | Phase 2 MVP Target | Measurement Tool |
|---|---|---|---|
| Average Handling Time (AHT) | 35–50 min | 20–25 min | POS Session Telemetry |
| Info Lookup Time | 3–5 min (across 5–10 apps) | <30 seconds | Sidecar Event Timers |
| Daily Active Rep Adoption | 0% | ≥80% of pilot reps | System Usage Audit Logs |
| Sales Conversion Rate | Baseline | +10–15% increase | POS Checkout Data vs. Control Stores |
| New Rep Onboarding Time | 4–6 weeks | 1–2 weeks | HR & Operations Assessments |

---

## 10. Engineering Approach: The 12-Layer Production AI Agent Stack

The MVP is built against a 12-layer production AI agent stack. Building the
agent is only one layer; production AI is the engineering system around it.
Tools change constantly — systems barely do. Understanding why each layer
exists and what failure it prevents lets us swap the model, framework,
database, vector store, or orchestration technology without losing the
architecture underneath it.

These are the failure modes the stack must handle:

- Missing and incomplete data before it reaches the agent
- Knowledge structuring and retrieval
- Context selection for different users and cases
- Task-specific model profiles
- Domain terminology and semantic resolution
- Multi-agent orchestration and conditional routing
- Generate → Verify → Retry → Escalate loops
- Repeatable AI agent evaluations
- Trace capture, replay, and run comparison
- Timeouts, retries, backoff, and fallbacks
- Recurring production feedback and controlled continual improvement

| # | Layer | Purpose (failure it prevents) | Omni-Connect Phase 2 application |
|---|---|---|---|
| 1 | **Business Understanding** | Defines the decision, constraints, ownership, and success criteria | Sections 1–5 above: pilot scope, AHT/adoption gates, human-in-the-loop authority |
| 2 | **Data Understanding** | Profile completeness, missingness, quality, freshness, provenance | `data/` + `docs/data.md` schemas; `usage_telemetry.json` currently pending — audit missingness before pilot |
| 3 | **Knowledge Engineering** | Turn raw information into structured, retrievable, traceable knowledge | Category B policy docs → `data/knowledge_base/` → FAISS + BM25 index in `data/kb_store/`; doc IDs + versions kept as citation trace |
| 4 | **Model Engineering** | Match model profiles to classification, generation, verification tasks | `config/llm_config.yaml`: chat (Anthropic/OpenRouter), local `bge-small-en-v1.5` embedding + `bge-reranker-base` |
| 5 | **Context Engineering** | Select the right subset of knowledge for each decision | `PromotionEvaluatorAgent` blockers vs. notes; `PolicyRetrieverAgent` top-k + threshold; customer-context assembly in agents |
| 6 | **Semantic Engineering** | Normalize terminology and resolve meaning (concepts, not raw strings) | Catalog IDs (plan/device/promo) as canonical ground truth; retrieval over the policy lexicon |
| 7 | **Agent Engineering** | Orchestrate agents, tools, state, and conditional workflow routing | `src/agents/*` specialists + `src/core/agent_coordinator.py` (`build_plan`/`run_workflow`) |
| 8 | **Loop Engineering** | Control what happens after every attempt: verify, correct, retry, stop, escalate | Phase 2 task: add verify→retry over RAG lookup misses and escalation-to-manager for excluded cases |
| 9 | **Evaluation Engineering** | Test expected behavior across representative cases, measure system-level performance | Phase 2 task: golden-question set over `rag_pipeline_runner.py query`; AHT/conversion gates (§9) |
| 10 | **Harness Engineering** | Capture complete runs so failures can be reproduced, inspected, replayed, compared | Phase 2 task: request/response capture for every Copilot call feeding the Audit & Telemetry Service (WS3) |
| 11 | **Infrastructure Engineering** | Handle timeouts, unavailable dependencies, retries, backoff, fallbacks, failures safely | `config/llm_config.yaml` (timeout/retries); Phase 2: fallback caching + <1.5 s latency SLO (§5.2) |
| 12 | **Continual Learning** | Turn recurring production feedback into evaluated, human-approved improvements | Phase 2 feedback widgets (👍/👎 + comments) → weekly prompt/RAG metric review → Executive Gate Review |

### How the stack maps onto the current codebase

| Layer | Current state (code) | Phase 2 delta |
|---|---|---|
| 3 Knowledge | `scripts/knowledge_data_generator.py`, `src/knowledge/*`, `data/kb_store/` | Add competitive battlecards; keep doc IDs/versions for citations |
| 5 Context | `src/agents/*` prompt assembly | `Context Assembly Engine` (Deliverable 2.1): live Customer 360 payload → prompt |
| 7 Agents | `AgentCoordinator` default linear plan | Conditional routing; wire the portal's scaffold recommendation + chat to the coordinator |
| 8 Loops | None | Verify→retry→escalate around RAG misses and eligibility blockers |
| 9 Eval | `WorkflowValidator` (structural) + manual `rag query` | Automated golden set + system-level metric harness |
| 10 Harness | None | Trace capture/replay for telemetry & debugging |
| 11 Infra | `timeout`/`max_retries` in LLM client | Timeout/backoff/fallback middleware + latency SLO monitoring |
| 12 Continual learning | Feedback surfaced via `app/portal.py` shortcuts only | In-app feedback widgets → weekly improvement loop |

### Implementation sequence within the 12-week sprint

| Sprint weeks | Focus | Stack layers exercised |
|---|---|---|
| 1–3 | Data Integration & API Wire-Up | 1, 2, 3, 11 |
| 4–6 | Prompt Engineering & UI Development | 4, 5, 6, 7 |
| 7–9 | Pilot Deployment & Store Training | 7, 8, 10 (feedback capture) |
| 10–12 | Telemetry Tracking & Gate Review | 8, 9, 12 |