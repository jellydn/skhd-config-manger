# Spec-Kit Methodology: What It Actually Recommends

Based on official spec-kit documentation research (2025-11-02)

## Core Philosophy

> **"Debugging means fixing specifications and their implementation plans that generate incorrect code."**

### Key Principles

1. **Specifications are Living Artifacts**
   - Specifications should be updated when requirements change
   - Feedback from implementation, testing, and production informs spec evolution
   - Specs drive code generation/regeneration, not the other way around

2. **Specification-First Problem Solving**
   - Bugs trace back to flawed specs, incomplete plans, or incorrect code generation
   - Fix the SOURCE artifact (spec/plan), not just the symptom (code)
   - Changes cascade: Spec → Plan → Tasks → Code

3. **Feedback Loops**
   ```
   Production/Testing → Specification Updates → Regeneration
   ```

---

## What Spec-Kit ACTUALLY Says

### ✅ Explicitly Covered

#### 1. **Greenfield Projects (0-to-1 Development)**

**Workflow:**
```bash
# Step 1: Establish project principles
/speckit.constitution Create principles focused on code quality, testing, UX

# Step 2: Create feature specification
/speckit.specify <description of what you want to build>

# Step 3: Generate implementation plan
/speckit.plan <tech stack and architecture decisions>

# Step 4: Optional - Clarify ambiguities
/speckit.clarify

# Step 5: Optional - Analyze consistency
/speckit.analyze

# Step 6: Generate task breakdown
/speckit.tasks

# Step 7: Implement
/speckit.implement
```

**Best For:**
- ✅ New projects starting from scratch
- ✅ Well-defined features with clear requirements
- ✅ Single-developer or small team projects
- ✅ Projects where you control the entire stack

#### 2. **Production Feedback Loop**

**Documented Approach:**
> "Production metrics and incidents don't just trigger hotfixes—they update specifications for the next regeneration."

**Feedback Types:**
1. **Operational Feedback**: Production data → specification updates
   - Performance bottlenecks → New non-functional requirements
   - Security vulnerabilities → New constraints
   - User behavior patterns → Refined acceptance criteria

2. **Specification-Driven Regeneration**: Spec changes → automatic code updates
   - "Modify a user story, and corresponding API endpoints regenerate"

3. **Exploratory Feedback**: Early implementations test if specs make sense
   - "An iterative dance between specification, implementation, and operational reality"

#### 3. **Requirement Changes**

**When to Update Specs:**
- ✅ User feedback changes acceptance criteria
- ✅ Market conditions shift requiring pivots
- ✅ Production incidents reveal missing constraints
- ✅ Performance bottlenecks become new NFRs
- ✅ Security vulnerabilities become constraints

**Quote:**
> "When a product manager updates acceptance criteria, implementation plans automatically flag affected technical decisions."

#### 4. **Brownfield Modernization**

**Mentioned but not detailed:**
- Spec-kit supports "brownfield modernization"
- No explicit workflow documented
- Implied: Create specs for existing systems, then regenerate

---

## What Spec-Kit DOES NOT Cover

### ❌ Explicitly Missing

#### 1. **PR Feedback and Code Review**

**What's Missing:**
- No documented workflow for handling PR comments
- No guidance on incorporating reviewer feedback into specs
- No process for code review → spec refinement loop

**Our Inference (What We Did):**
```
PR Feedback → Identify Spec Gaps → Update Specs → Fix Code
```

**Why This Makes Sense:**
- PR comments often reveal missing NFRs (accessibility, performance, security)
- These ARE specification gaps that should be documented
- Prevents future implementations from missing same requirements

#### 2. **Bug Fixing During Implementation**

**What's Missing:**
- No step-by-step bug fix workflow
- No decision tree: "Update spec vs just fix code"
- No guidance on when bugs are spec issues vs code issues

**Our Inference:**
- **Spec Bug**: Missing requirement, wrong acceptance criteria, incomplete plan
  - Action: Update spec → regenerate/fix code
- **Code Bug**: Correct spec, incorrect implementation
  - Action: Fix code (spec already correct)
- **Discovered Requirement**: Something not specified but needed
  - Action: Update spec with new requirement → fix code

#### 3. **Iterative Development**

**What's Missing:**
- No workflow for agile/iterative sprints
- No guidance on incremental spec updates vs big-bang specs
- No process for "spec debt" (code ahead of specs)

**Problem:**
- Spec-kit assumes specs BEFORE implementation
- Real world: Requirements emerge during implementation
- No documented way to handle this mismatch

#### 4. **Team Collaboration**

**What's Missing:**
- No multi-developer workflows
- No spec review/approval processes
- No conflict resolution when specs and code diverge
- No handoff procedures (designer → developer → QA)

#### 5. **Spec-to-Code Divergence**

**What Spec-Kit Claims:**
> "When specifications and implementation plans generate code, there is no gap—only transformation."

**Reality We Found:**
- Specs get stale quickly in iterative development
- Manual coding (not generation) creates divergence
- No practical guidance on maintaining sync

---

## Practical Application Guide

### When Spec-Kit Works Well

**✅ Use Spec-Kit Strictly For:**

1. **Greenfield Features**
   - Clear, well-understood requirements
   - Can specify upfront completely
   - Single developer or tight team

2. **Major Refactors/Rewrites**
   - Replacing entire systems
   - Time to plan thoroughly
   - Can regenerate code from specs

3. **Compliance/Regulated Projects**
   - Need audit trail of requirements
   - Specs must stay current with code
   - Documentation is critical

### When to Use Pragmatically

**🟡 Adapt Spec-Kit For:**

1. **Ongoing Product Development**
   - Use for initial planning only
   - Specs become historical reference
   - Accept spec-code divergence

2. **Agile/Iterative Teams**
   - Use `/speckit.specify` for feature kickoff
   - Use `/speckit.plan` for architecture decisions
   - Skip `/speckit.implement` (code manually)
   - Update specs only for major requirement changes

3. **Fast-Moving Startups**
   - Specs for MVP features
   - Don't maintain spec-code sync
   - Revisit specs only for major pivots

---

## Our Recommended Hybrid Approach

Based on what spec-kit DOES and DOESN'T cover:

### Phase 1: Planning (Strict Spec-Kit)
```bash
/speckit.specify <feature>
/speckit.clarify  # if ambiguous
/speckit.plan <tech approach>
/speckit.tasks
```

### Phase 2: Implementation (Pragmatic)
```bash
# Code normally, iterate based on feedback
# Don't update specs for every small change
# Focus on working software
```

### Phase 3: PR Review (Spec-Kit Informed)
```bash
# IF PR reveals missing requirements:
#   → Update spec with discovered NFRs
#   → Document for future features

# IF PR suggests code improvements only:
#   → Just fix code, don't update specs
```

### Phase 4: Production Feedback (Spec-Kit)
```bash
# IF production issues reveal spec gaps:
#   → Update specs
#   → Document new constraints
#   → Inform next iteration planning
```

---

## Decision Framework

### "Should I Update the Spec?"

**✅ YES - Update Spec When:**

- [ ] Missing requirement discovered (accessibility, security, performance)
- [ ] Acceptance criteria were incomplete/wrong
- [ ] New constraint needed for all future features
- [ ] Production incident reveals spec gap
- [ ] API contract needs documentation
- [ ] Compliance/audit requirement

**❌ NO - Just Fix Code When:**

- [ ] Typo or syntax error
- [ ] Better variable naming
- [ ] Code refactoring (same behavior)
- [ ] Performance optimization (meets existing spec)
- [ ] Bug in implementation logic (spec was correct)
- [ ] UI polish/styling tweaks

**🤔 MAYBE - Use Judgment When:**

- [ ] PR comment about code style
- [ ] "Nice to have" improvements
- [ ] Technical debt cleanup
- [ ] Non-critical edge cases

---

## Handling Different Scenarios

### Scenario 1: PR Comment "Missing aria-labels"

**Spec-Kit Approach:**
```
1. Recognize: Spec gap (accessibility requirement missing)
2. Update spec: Add NFR-A01: "All icon buttons MUST have aria-labels"
3. Fix code: Add aria-labels
4. Update checklist: Mark accessibility requirements complete
5. Result: Future features won't miss this requirement
```

### Scenario 2: Bug "Button doesn't work on click"

**Spec-Kit Approach:**
```
1. Analyze: Is spec wrong or code wrong?
   - Spec says: "Button MUST trigger action on click" ✓
   - Code: onclick handler missing ✗
2. Diagnosis: Code bug (spec was correct)
3. Fix code only, don't update spec
```

### Scenario 3: Production Issue "App crashes on large files"

**Spec-Kit Approach:**
```
1. Recognize: Missing NFR (no file size limit specified)
2. Update spec: Add NFR-P04: "System MUST handle files up to 10MB"
3. Update plan: Document pagination/chunking approach
4. Fix code: Implement file size validation + chunking
5. Update checklist: Add performance requirement
```

### Scenario 4: Feature Request "Can we add dark mode?"

**Spec-Kit Approach:**
```
1. New feature, not bug fix
2. Create NEW spec: /speckit.specify "Add dark mode theme support"
3. Follow full workflow: specify → plan → tasks → implement
4. Don't retrofit into existing feature specs
```

---

## Lessons from Our Experience

### What Worked Well
- ✅ Initial feature planning with `/speckit.specify`
- ✅ Structured task breakdown with `/speckit.tasks`
- ✅ Updating specs when PR revealed missing NFRs
- ✅ Documentation of discovered requirements

### What Didn't Work
- ❌ Keeping specs perfectly synced with iterative changes
- ❌ Updating specs for every small code improvement
- ❌ Treating specs as "source of truth" for rapidly evolving code

### Key Insight
**Spec-kit is a PLANNING framework, not a MAINTENANCE framework.**

Use it to:
- ✅ Create thoughtful initial specs
- ✅ Document architectural decisions
- ✅ Capture discovered requirements
- ✅ Maintain requirement history

Don't try to:
- ❌ Keep specs perfectly synced with code
- ❌ Update specs for trivial changes
- ❌ Use specs as real-time code documentation

---

## Conclusion

**Spec-Kit's Core Value:**
- Forcing upfront thinking about requirements
- Structured documentation of decisions
- Feedback loop from implementation → specification

**Spec-Kit's Limitations:**
- Assumes specs can be complete before implementation (rarely true)
- No guidance for real-world PR review workflows
- Doesn't address spec-code divergence in iterative development
- Built for ideal waterfall-ish flow, not agile chaos

**Best Practice:**
Use spec-kit for what it's good at (planning), accept its limitations (maintenance), and fill gaps with pragmatic judgment.

---

**Created**: 2025-11-02
**Based On**: Official spec-kit documentation from github.com/github/spec-kit
**Context**: Lessons learned from implementing Feature 003 (Shortcut Duplicate)
