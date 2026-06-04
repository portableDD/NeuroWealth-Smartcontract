# Design Document: GitHub Issue Templates

## Overview

This design specifies the implementation of GitHub issue templates for the NeuroWealth-Smartcontract repository. The system consists of two standardized markdown templates (bug report and feature request), a GitHub configuration file, and integration with the project's CONTRIBUTING.md guide.

The templates are static markdown files stored in `.github/ISSUE_TEMPLATE/` that GitHub automatically detects and displays when users create new issues. This design ensures templates are discoverable, maintainable, and aligned with the project's development standards.

### Key Design Principles

1. **GitHub-Native**: Leverage GitHub's built-in template system without custom tooling
2. **Markdown-Based**: Store templates as version-controlled markdown files for easy updates
3. **Guidance-Rich**: Include clear instructions and examples to help contributors provide complete information
4. **Project-Aligned**: Reference existing project documentation (CONTRIBUTING.md, SECURITY.md, ERROR_STYLE_GUIDE.md, ARCHITECTURE.md, EVENTS.md)
5. **Maintainable**: Simple structure that can be updated via pull requests like any other project file

---

## Architecture

### System Components

```
GitHub Issue Creation Flow
    ↓
GitHub detects .github/ISSUE_TEMPLATE/ directory
    ↓
GitHub reads config.yml for template configuration
    ↓
GitHub displays template options to user
    ↓
User selects bug_report.md or feature_request.md
    ↓
GitHub pre-populates issue form with template content
    ↓
User fills out template sections
    ↓
Issue is created with structured information
```

### File Structure

```
.github/
├── ISSUE_TEMPLATE/
│   ├── config.yml                 # GitHub template configuration
│   ├── bug_report.md              # Bug report template
│   └── feature_request.md         # Feature request template
```

### Integration Points

1. **CONTRIBUTING.md**: Updated to reference issue templates and explain when to use each
2. **SECURITY.md**: Referenced in both templates for security issue reporting
3. **ERROR_STYLE_GUIDE.md**: Referenced in bug template for error message standards
4. **ARCHITECTURE.md**: Referenced in both templates for system design context
5. **EVENTS.md**: Referenced in bug template for event-related issues

---

## Components and Interfaces

### Component 1: Bug Report Template (`bug_report.md`)

**Purpose**: Standardize bug reports with all necessary information for reproduction and debugging

**Sections**:

1. **Title Field**
   - Pre-filled with "Bug: "
   - Guides user to provide concise bug summary
   - Example: "Bug: Deposit fails with insufficient liquidity error on testnet"

2. **Description**
   - Guidance: "Provide a clear and concise description of the bug"
   - Explains what the bug is and its impact
   - Distinguishes from "Expected Behavior" and "Actual Behavior"

3. **Soroban Network**
   - Dropdown or text field for network selection
   - Options: devnet, testnet, mainnet
   - Includes explanation of each network
   - Guidance: "Which Soroban network did you encounter this bug on?"

4. **Contract ID**
   - Dedicated field for contract identifier
   - Includes example format: `CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4`
   - Instructions to find Contract ID from deployment logs or StellarExpert
   - Guidance: "Provide the contract ID where the bug occurs (if applicable)"

5. **Contract Function**
   - Field for the specific function affected (e.g., `deposit`, `withdraw`, `rebalance`)
   - Guidance: "Which contract function is affected?"

6. **Contract Parameters**
   - Field for parameters/arguments used when calling the function
   - Guidance: "What parameters or arguments were used?"

7. **Steps to Reproduce**
   - Numbered list format
   - Guidance: "Provide step-by-step instructions to reproduce the bug"
   - Example structure:
     ```
     1. Run `cargo test --verbose`
     2. Execute `stellar contract invoke --id <CONTRACT_ID> -- deposit 1000`
     3. Observe the error
     ```

8. **Expected Behavior**
   - Guidance: "Describe what you expected to happen"
   - Separate from "Actual Behavior" for clarity

9. **Actual Behavior**
   - Guidance: "Describe what actually happened"
   - Separate from "Expected Behavior" for clarity

10. **Test Command Output**
    - Dedicated section for reproducible test commands
    - Instructions to run failing test and paste full output
    - Common examples:
      - `cargo test --verbose`
      - `stellar contract invoke --id <CONTRACT_ID> -- <function> <args>`
      - `npm test`
    - Guidance: "Use code blocks (triple backticks) for formatting"

11. **Environment Details**
    - Rust version: `rustc --version`
    - Stellar CLI version: `stellar --version`
    - Node.js version: `node --version` (if applicable)
    - Operating system and version
    - Soroban network (devnet/testnet/mainnet)
    - Guidance: "Provide your environment information to help identify version-specific bugs"

12. **Verification Checklist**
    - Checkbox: "I have run `cargo fmt` and `cargo clippy`"
    - Checkbox: "I have run the full test suite (`cargo test`)"
    - Checkbox: "I have verified this is not a security issue (see SECURITY.md)"
    - Checkbox: "I have provided all required information above"

13. **Additional Context**
    - Optional field for screenshots, links, or related discussions
    - Guidance: "Add any other context about the problem here"

14. **Transaction Hash** (if applicable)
    - Field for on-chain transaction hash
    - Guidance: "If this is an on-chain issue, provide the transaction hash"

15. **Security Issue Notice**
    - Prominent notice: "⚠️ If this is a security issue, please report it via [SECURITY.md](SECURITY.md) instead of creating a public issue"
    - Checkbox: "Is this a security issue?"

### Component 2: Feature Request Template (`feature_request.md`)

**Purpose**: Standardize feature requests with clear articulation of value and implementation approach

**Sections**:

1. **Title Field**
   - Pre-filled with "Feature: "
   - Guides user to provide concise feature summary
   - Example: "Feature: Add withdrawal queue for liquidity management"

2. **Feature Description**
   - Guidance: "Provide a clear and concise description of the feature"
   - Explains what the feature does and its purpose
   - Distinguishes from "Use Case and Motivation"

3. **Use Case and Motivation**
   - Guidance: "Explain why this feature is needed and what problem it solves"
   - Helps maintainers understand the value proposition
   - Encourages thinking about real-world scenarios

4. **Proposed Solution**
   - Guidance: "Describe your proposed implementation approach"
   - Encourages technical thinking about how to implement
   - Can include pseudocode or architecture sketches

5. **Alternative Approaches**
   - Guidance: "Discuss other possible solutions or approaches"
   - Helps evaluate trade-offs
   - Encourages comprehensive thinking

6. **Security Considerations**
   - Guidance: "If this feature involves smart contract changes, discuss security implications"
   - References ERROR_STYLE_GUIDE.md for error handling
   - References SECURITY.md for security model
   - Guidance: "Consider access control, state changes, and potential attack vectors"

7. **Network-Specific Considerations**
   - Guidance: "If this feature involves Soroban network interactions, discuss network-specific considerations"
   - Considerations for devnet vs testnet vs mainnet
   - Guidance: "Consider deployment, testing, and production implications"

8. **Architecture Alignment**
   - Guidance: "How does this feature align with the project architecture?"
   - References ARCHITECTURE.md
   - Encourages understanding of system design

9. **Additional Context**
   - Optional field for links, screenshots, or related discussions
   - Guidance: "Add any other context about the feature here"

10. **Related Issues or PRs**
    - Field for linking to existing discussions
    - Guidance: "Link to any related issues or pull requests"

11. **Completeness Checklist**
    - Checkbox: "I have reviewed the CONTRIBUTING.md guide"
    - Checkbox: "I have reviewed the ARCHITECTURE.md documentation"
    - Checkbox: "I have considered security implications"
    - Checkbox: "I have provided all recommended information above"

---

## Data Models

### Template File Structure

Each template is a markdown file with the following structure:

```markdown
---
name: Template Name
about: Brief description of template purpose
title: "Prefix: "
labels: [label1, label2]
assignees: []
---

## Section 1
Guidance text and instructions

## Section 2
Guidance text and instructions

...
```

### YAML Front Matter

The YAML front matter (between `---` markers) contains GitHub-specific metadata:

- **name**: Display name for the template in GitHub UI
- **about**: Description shown when user selects template
- **title**: Pre-filled title prefix (e.g., "Bug: " or "Feature: ")
- **labels**: Auto-applied labels when issue is created
- **assignees**: Optional auto-assigned reviewers

### Markdown Content

Template content uses standard GitHub-flavored markdown:

- Headers (`##`) for section organization
- Bold text (`**text**`) for emphasis
- Code blocks (triple backticks) for examples
- Checkboxes (`- [ ]`) for verification lists
- Links (`[text](url)`) for references

---

## Configuration

### config.yml Structure

```yaml
# Disable blank issues
blank_issues_enabled: false

# Contact links for common questions
contact_links:
  - name: Security Issues
    url: https://github.com/NeuroWealth/NeuroWealth-Smartcontract/security/policy
    about: Report security vulnerabilities responsibly
  - name: Documentation
    url: https://github.com/NeuroWealth/NeuroWealth-Smartcontract/blob/develop/CONTRIBUTING.md
    about: Review contribution guidelines and development setup
  - name: Architecture
    url: https://github.com/NeuroWealth/NeuroWealth-Smartcontract/blob/develop/ARCHITECTURE.md
    about: Understand the system architecture
```

**Configuration Details**:

- **blank_issues_enabled**: Set to `false` to require users to select a template
- **contact_links**: Provides quick access to important resources
  - Security policy link for security-related questions
  - CONTRIBUTING.md link for development setup questions
  - ARCHITECTURE.md link for architecture questions

---

## File Organization

### Directory Structure

```
.github/
├── ISSUE_TEMPLATE/
│   ├── config.yml
│   ├── bug_report.md
│   └── feature_request.md
├── workflows/
│   └── ci.yml
└── ...
```

### File Placement Rationale

- **`.github/ISSUE_TEMPLATE/`**: GitHub's standard location for issue templates
  - GitHub automatically detects this directory
  - Templates are discoverable without manual configuration
  - Follows GitHub's recommended structure

- **`config.yml`**: GitHub's standard configuration file for templates
  - Controls template behavior (blank issues, contact links)
  - Must be in the same directory as templates

### Version Control

- All template files are version-controlled in Git
- Changes to templates go through pull request review process
- Template history is preserved in Git commit log
- Easy to revert template changes if needed

---

## Integration Points

### Integration with CONTRIBUTING.md

**New Section**: "Reporting Issues"

The CONTRIBUTING.md file will be updated with a new section that:

1. Explains when to use bug report template vs feature request template
2. Provides links to the issue templates
3. References the security policy for security-related issues
4. Explains issue labels and their meanings
5. Provides guidance on issue quality expectations

**Content Structure**:

```markdown
## Reporting Issues

### Bug Reports

Use the [bug report template](/.github/ISSUE_TEMPLATE/bug_report.md) when:
- You've found a defect in the smart contract
- You've encountered unexpected behavior
- You have a reproducible test case

### Feature Requests

Use the [feature request template](/.github/ISSUE_TEMPLATE/feature_request.md) when:
- You want to propose a new capability
- You want to suggest an enhancement
- You have an idea for improving the project

### Security Issues

For security-related issues, please follow the [Security Policy](SECURITY.md) 
and report via GitHub's security advisory system instead of creating a public issue.

### Issue Labels

We use the following labels to categorize issues:
- `bug`: Something isn't working as expected
- `enhancement`: New feature or request
- `documentation`: Improvements or additions to documentation
- `good first issue`: Good for newcomers
- `security`: Security-related issues or improvements
- `help wanted`: Extra attention needed
```

### References to Project Documentation

**Bug Report Template References**:
- CONTRIBUTING.md: Development setup and CI requirements
- SECURITY.md: Security policy and responsible disclosure
- ERROR_STYLE_GUIDE.md: Error message standards
- ARCHITECTURE.md: System components and design
- EVENTS.md: Event emission requirements

**Feature Request Template References**:
- CONTRIBUTING.md: Development setup and contribution process
- SECURITY.md: Security model and trust model
- ARCHITECTURE.md: System design and components
- ERROR_STYLE_GUIDE.md: Error handling standards

---

## Implementation Details

### Bug Report Template Content

**File**: `.github/ISSUE_TEMPLATE/bug_report.md`

**YAML Front Matter**:
```yaml
---
name: Bug Report
about: Report a defect in the smart contract or related systems
title: "Bug: "
labels: ["bug"]
assignees: []
---
```

**Template Sections** (in order):

1. **Description Section**
   ```markdown
   ## Description
   
   Provide a clear and concise description of the bug. What is the issue?
   ```

2. **Soroban Network Section**
   ```markdown
   ## Soroban Network
   
   Which Soroban network did you encounter this bug on?
   
   - [ ] Devnet (local development network)
   - [ ] Testnet (public test network)
   - [ ] Mainnet (production network)
   
   **Why this matters**: Network-specific bugs help us identify environment-related issues.
   ```

3. **Contract Information Section**
   ```markdown
   ## Contract Information
   
   ### Contract ID
   
   Provide the contract ID where the bug occurs (if applicable).
   
   Example format: `CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4`
   
   **How to find it**:
   - Check deployment logs
   - Look it up on [StellarExpert](https://stellar.expert/)
   - Use `stellar contract info --id <CONTRACT_ID>`
   
   ### Contract Function
   
   Which contract function is affected? (e.g., `deposit`, `withdraw`, `rebalance`)
   
   ### Contract Parameters
   
   What parameters or arguments were used when calling the function?
   ```

4. **Steps to Reproduce Section**
   ```markdown
   ## Steps to Reproduce
   
   Provide step-by-step instructions to reproduce the bug:
   
   1. 
   2. 
   3. 
   
   **Example**:
   ```
   1. Run `cargo test --verbose`
   2. Execute `stellar contract invoke --id <CONTRACT_ID> -- deposit 1000`
   3. Observe the error
   ```
   ```

5. **Expected vs Actual Behavior Section**
   ```markdown
   ## Expected Behavior
   
   Describe what you expected to happen.
   
   ## Actual Behavior
   
   Describe what actually happened instead.
   ```

6. **Test Command Output Section**
   ```markdown
   ## Test Command Output
   
   Provide the output of the failing test or command. This is essential for debugging.
   
   ### Command Used
   
   What command did you run?
   
   Examples:
   - `cargo test --verbose`
   - `stellar contract invoke --id <CONTRACT_ID> -- <function> <args>`
   - `npm test`
   
   ### Full Output
   
   Paste the complete output (including error messages and stack traces):
   
   ```
   [Paste output here]
   ```
   
   **Tip**: Use code blocks (triple backticks) for formatting.
   ```

7. **Environment Details Section**
   ```markdown
   ## Environment Details
   
   Provide your environment information to help identify version-specific bugs.
   
   ### Versions
   
   - Rust version: `rustc --version`
   - Stellar CLI version: `stellar --version`
   - Node.js version: `node --version` (if applicable)
   - Cargo version: `cargo --version`
   
   ### System Information
   
   - Operating system and version: (e.g., macOS 14.0, Ubuntu 22.04, Windows 11)
   - Architecture: (e.g., x86_64, ARM64)
   
   ### Soroban Network
   
   - Network: (devnet/testnet/mainnet)
   - RPC endpoint: (if using custom endpoint)
   ```

8. **Error Message Section**
   ```markdown
   ## Error Message
   
   If you received an error message, please provide it here.
   
   **Note**: Review the [Error Message Style Guide](ERROR_STYLE_GUIDE.md) to understand error categories.
   
   ```
   [Paste exact error message here]
   ```
   ```

9. **Verification Checklist Section**
   ```markdown
   ## Verification Checklist
   
   Before submitting, please verify:
   
   - [ ] I have run `cargo fmt` and `cargo clippy` on my changes
   - [ ] I have run the full test suite (`cargo test`)
   - [ ] I have provided all required information above
   - [ ] I have reviewed the [CONTRIBUTING.md](CONTRIBUTING.md) guide
   - [ ] This is not a security issue (see [SECURITY.md](SECURITY.md))
   
   **Note**: Incomplete issues may be closed and asked to be resubmitted.
   ```

10. **Additional Context Section**
    ```markdown
    ## Additional Context
    
    Add any other context about the problem here (screenshots, links, related discussions, etc.).
    ```

11. **Security Notice Section**
    ```markdown
    ## Security Notice
    
    ⚠️ **If this is a security issue**, please report it via [SECURITY.md](SECURITY.md) 
    instead of creating a public issue. Do not disclose security vulnerabilities publicly.
    
    - [ ] This is a security issue (report via SECURITY.md instead)
    ```

### Feature Request Template Content

**File**: `.github/ISSUE_TEMPLATE/feature_request.md`

**YAML Front Matter**:
```yaml
---
name: Feature Request
about: Propose a new capability or enhancement
title: "Feature: "
labels: ["enhancement"]
assignees: []
---
```

**Template Sections** (in order):

1. **Feature Description Section**
   ```markdown
   ## Feature Description
   
   Provide a clear and concise description of the feature. What does it do?
   ```

2. **Use Case and Motivation Section**
   ```markdown
   ## Use Case and Motivation
   
   Explain why this feature is needed and what problem it solves.
   
   **Example**: "Currently, users cannot withdraw funds if the vault has fully deployed 
   its USDC to external protocols. This feature would implement a withdrawal queue to 
   handle this scenario."
   ```

3. **Proposed Solution Section**
   ```markdown
   ## Proposed Solution
   
   Describe your proposed implementation approach. How would you implement this feature?
   
   You can include:
   - High-level architecture
   - Pseudocode or algorithm sketches
   - Data structure changes
   - New functions or methods
   ```

4. **Alternative Approaches Section**
   ```markdown
   ## Alternative Approaches
   
   Discuss other possible solutions or approaches. What are the trade-offs?
   
   **Example**: 
   - Approach A: Implement withdrawal queue (more complex, better UX)
   - Approach B: Require users to wait for rebalance (simpler, worse UX)
   ```

5. **Security Considerations Section**
   ```markdown
   ## Security Considerations
   
   If this feature involves smart contract changes, discuss security implications.
   
   **Consider**:
   - Access control: Who can call this function?
   - State changes: What contract state is modified?
   - Attack vectors: What could go wrong?
   - Error handling: How should errors be handled?
   
   **References**:
   - [SECURITY.md](SECURITY.md) - Security model and trust model
   - [ERROR_STYLE_GUIDE.md](ERROR_STYLE_GUIDE.md) - Error handling standards
   - [ARCHITECTURE.md](ARCHITECTURE.md) - System design
   ```

6. **Network-Specific Considerations Section**
   ```markdown
   ## Network-Specific Considerations
   
   If this feature involves Soroban network interactions, discuss network-specific considerations.
   
   **Consider**:
   - Devnet: Local development and testing
   - Testnet: Integration testing and public testing
   - Mainnet: Production deployment and real users
   
   **Example**: "This feature requires different configuration for devnet vs mainnet 
   due to different RPC endpoints and contract deployments."
   ```

7. **Architecture Alignment Section**
   ```markdown
   ## Architecture Alignment
   
   How does this feature align with the project architecture?
   
   **Reference**: [ARCHITECTURE.md](ARCHITECTURE.md)
   
   Explain how this feature fits into the existing system design.
   ```

8. **Additional Context Section**
   ```markdown
   ## Additional Context
   
   Add any other context about the feature here (links, screenshots, related discussions, etc.).
   ```

9. **Related Issues or PRs Section**
   ```markdown
   ## Related Issues or PRs
   
   Link to any related issues or pull requests:
   
   - Closes: #
   - Related to: #
   - Depends on: #
   ```

10. **Completeness Checklist Section**
    ```markdown
    ## Completeness Checklist
    
    Before submitting, please verify:
    
    - [ ] I have reviewed the [CONTRIBUTING.md](CONTRIBUTING.md) guide
    - [ ] I have reviewed the [ARCHITECTURE.md](ARCHITECTURE.md) documentation
    - [ ] I have considered security implications
    - [ ] I have provided all recommended information above
    
    **Note**: More complete feature requests are more likely to be implemented.
    ```

---

## Error Handling

### Template Validation

While GitHub doesn't provide built-in validation for template content, the design ensures:

1. **Clear Guidance**: Each section includes explicit guidance text
2. **Examples**: Sections include concrete examples of expected input
3. **Checklists**: Verification checklists encourage completeness
4. **References**: Links to project documentation guide users to relevant information

### User Guidance

**For Bug Reports**:
- Guidance text explains why each field is important
- Examples show what good bug reports look like
- Verification checklist ensures completeness before submission
- Security notice prevents accidental public disclosure of vulnerabilities

**For Feature Requests**:
- Guidance text explains what information is most helpful
- Examples show what good feature requests look like
- Completeness checklist encourages thorough thinking
- References to architecture and security documentation guide design thinking

### Issue Triage

The structured templates enable better triage:

1. **Bug Reports**: Consistent information enables faster reproduction and debugging
2. **Feature Requests**: Structured information enables better evaluation of value and feasibility
3. **Labels**: Auto-applied labels help categorize issues
4. **Contact Links**: Provide quick access to relevant documentation

---

## Testing Strategy

### Assessment of Property-Based Testing Applicability

This feature involves **static file creation and configuration**, not pure logic suitable for property-based testing. The templates are:

- **Markdown files**: Static content, not executable code
- **Configuration files**: YAML configuration, not algorithmic logic
- **GitHub integration**: External service integration, not internal logic

**Conclusion**: Property-based testing is **NOT applicable** to this feature.

### Testing Approach

Instead of property-based testing, the implementation uses:

1. **File Existence Tests**: Verify template files are created in correct locations
2. **Content Validation Tests**: Verify template content includes required sections
3. **YAML Validation Tests**: Verify config.yml is valid YAML
4. **Markdown Validation Tests**: Verify templates are valid markdown
5. **Integration Tests**: Verify templates work with GitHub's system
6. **Manual Testing**: Verify templates display correctly in GitHub UI

### Unit Tests

**Test Categories**:

1. **File Structure Tests**
   - Verify `.github/ISSUE_TEMPLATE/` directory exists
   - Verify `bug_report.md` file exists
   - Verify `feature_request.md` file exists
   - Verify `config.yml` file exists

2. **Content Validation Tests**
   - Verify bug report template includes all required sections
   - Verify feature request template includes all required sections
   - Verify templates include guidance text
   - Verify templates include examples
   - Verify templates include checklists

3. **YAML Validation Tests**
   - Verify config.yml is valid YAML
   - Verify config.yml has required fields
   - Verify config.yml has correct values

4. **Markdown Validation Tests**
   - Verify templates are valid markdown
   - Verify templates have correct heading structure
   - Verify templates have valid code blocks
   - Verify templates have valid links

5. **Integration Tests**
   - Verify templates are discoverable by GitHub
   - Verify templates display correctly in GitHub UI
   - Verify templates pre-populate issue form correctly
   - Verify auto-applied labels work correctly

### Manual Testing

**GitHub UI Testing**:

1. Navigate to repository on GitHub
2. Click "New Issue"
3. Verify both templates are displayed as options
4. Select bug report template
5. Verify template content is pre-populated
6. Verify title is pre-filled with "Bug: "
7. Verify labels are auto-applied
8. Repeat for feature request template

**Content Verification**:

1. Verify all sections are present
2. Verify guidance text is clear
3. Verify examples are helpful
4. Verify links work correctly
5. Verify code blocks render correctly
6. Verify checklists are functional

---

## Maintenance and Updates

### Template Evolution

Templates can be updated via pull requests:

1. **Minor Updates**: Clarify guidance text, fix typos, update examples
2. **Section Additions**: Add new sections based on feedback
3. **Reference Updates**: Update links to project documentation
4. **Label Changes**: Modify auto-applied labels based on project needs

### Version Control

- All template changes are tracked in Git
- Pull request review process ensures quality
- Commit history provides audit trail of changes
- Easy to revert changes if needed

### Documentation

- Template structure documented in this design document
- CONTRIBUTING.md references templates and explains usage
- Comments in template files explain purpose of each section
- GitHub's template documentation provides additional reference

---

## Summary

This design provides a comprehensive, maintainable system for GitHub issue templates that:

1. **Standardizes Issue Reporting**: Bug reports and feature requests follow consistent structure
2. **Improves Information Quality**: Guidance text and examples help contributors provide complete information
3. **Aligns with Project Standards**: Templates reference existing project documentation
4. **Enables Better Triage**: Structured information enables faster issue categorization and response
5. **Supports Project Evolution**: Templates can be easily updated via pull requests
6. **Leverages GitHub Native Features**: Uses GitHub's built-in template system without custom tooling

The implementation is straightforward: create three files in `.github/ISSUE_TEMPLATE/` (two markdown templates and one YAML configuration file) and update CONTRIBUTING.md to reference the templates. This provides immediate value to contributors and maintainers with minimal maintenance overhead.
