# Requirements Document

## GitHub Issue Templates Requirements

## Introduction

This document specifies requirements for implementing GitHub issue templates in the NeuroWealth-Smartcontract repository. The templates will standardize issue reporting for bug reports and feature requests, ensuring contributors provide essential information about Soroban network context, contract deployment details, and reproducible test cases. This improves issue triage efficiency and accelerates debugging and feature development.

## Glossary

- **System**: The GitHub issue template system that processes and displays templates when users create new issues
- **Bug_Reporter**: A contributor or user reporting a defect in the smart contract or related systems
- **Feature_Requester**: A contributor or user proposing a new capability or enhancement
- **Issue_Template**: A pre-formatted markdown file in `.github/ISSUE_TEMPLATE/` that GitHub displays as a form when creating issues
- **Soroban_Network**: The blockchain network context (devnet, testnet, or mainnet) where the contract is deployed
- **Contract_ID**: The unique identifier of a deployed Soroban smart contract on a specific network
- **Test_Command**: A reproducible shell command (e.g., `cargo test`, `stellar contract invoke`) that demonstrates the issue
- **Environment_Details**: System information including Rust version, Stellar CLI version, Node.js version, and operating system
- **Acceptance_Criteria**: Measurable conditions that verify a requirement is satisfied
- **Markdown_Format**: GitHub-flavored markdown syntax used for issue templates
- **Form_Fields**: Structured input sections in GitHub issue templates that guide users to provide specific information

## Requirements

### Requirement 1: Bug Report Template Structure

**User Story:** As a Bug_Reporter, I want a standardized bug report template, so that I can provide all necessary information for developers to reproduce and fix the issue.

#### Acceptance Criteria

1. WHEN a Bug_Reporter creates a new issue, THE System SHALL display a bug report template with the following sections:
   - Title field (pre-filled with "Bug: ")
   - Description of the bug
   - Soroban network (devnet/testnet/mainnet)
   - Contract ID
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Test command output
   - Environment details (Rust version, Stellar CLI version, Node.js version, OS)
   - Additional context

2. WHEN a Bug_Reporter fills out the template, THE System SHALL preserve all markdown formatting and code blocks in the submitted issue

3. WHEN a Bug_Reporter views the template, THE System SHALL display helpful guidance text explaining what information to provide in each field

4. WHERE the bug is related to contract deployment, THE System SHALL include a dedicated field for Contract_ID with an example format

5. WHERE the bug involves test failures, THE System SHALL include a dedicated field for Test_Command with instructions to paste the full command output

### Requirement 2: Feature Request Template Structure

**User Story:** As a Feature_Requester, I want a standardized feature request template, so that I can clearly communicate the feature idea, use case, and proposed implementation approach.

#### Acceptance Criteria

1. WHEN a Feature_Requester creates a new issue, THE System SHALL display a feature request template with the following sections:
   - Title field (pre-filled with "Feature: ")
   - Feature description
   - Use case and motivation
   - Proposed solution
   - Alternative approaches
   - Additional context
   - Related issues or PRs

2. WHEN a Feature_Requester fills out the template, THE System SHALL preserve all markdown formatting and code blocks in the submitted issue

3. WHEN a Feature_Requester views the template, THE System SHALL display helpful guidance text explaining what information to provide in each field

4. WHERE the feature involves smart contract changes, THE System SHALL include guidance about contract security implications

5. WHERE the feature involves Soroban network interactions, THE System SHALL include guidance about network-specific considerations (devnet vs testnet vs mainnet)

### Requirement 3: Template File Organization

**User Story:** As a repository maintainer, I want issue templates organized in a standard GitHub directory structure, so that GitHub automatically detects and displays them to users.

#### Acceptance Criteria

1. THE System SHALL create a `.github/ISSUE_TEMPLATE/` directory in the repository root

2. THE System SHALL create a `bug_report.md` file in `.github/ISSUE_TEMPLATE/` containing the bug report template

3. THE System SHALL create a `feature_request.md` file in `.github/ISSUE_TEMPLATE/` containing the feature request template

4. THE System SHALL create a `config.yml` file in `.github/ISSUE_TEMPLATE/` that configures template behavior (blank template disabled, contact links)

5. WHEN the `.github/ISSUE_TEMPLATE/` directory is created with both template files, THE System SHALL ensure GitHub processes the directory and makes both templates available as options when users click "New Issue"

### Requirement 4: Bug Report Template Content

**User Story:** As a Bug_Reporter, I want clear instructions and examples in the bug template, so that I understand what information is most helpful for debugging.

#### Acceptance Criteria

1. THE Bug_Report_Template SHALL include a section explaining how to identify the Soroban_Network (devnet/testnet/mainnet) and why it matters

2. THE Bug_Report_Template SHALL include an example Contract_ID format (e.g., `CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4`)

3. THE Bug_Report_Template SHALL include instructions to run `cargo test --verbose` and paste the full output

4. THE Bug_Report_Template SHALL include instructions to run `stellar --version` and `rustc --version` and paste the output

5. THE Bug_Report_Template SHALL include a checklist of common verification steps (e.g., "I have run `cargo fmt` and `cargo clippy`", "I have run the full test suite")

6. THE Bug_Report_Template SHALL include a section for "Steps to Reproduce" with numbered sub-steps

7. THE Bug_Report_Template SHALL include separate sections for "Expected Behavior" and "Actual Behavior" to clarify the discrepancy

### Requirement 5: Feature Request Template Content

**User Story:** As a Feature_Requester, I want clear instructions and examples in the feature template, so that I can articulate the feature idea and its value.

#### Acceptance Criteria

1. THE Feature_Request_Template SHALL include a section for "Feature Description" with guidance to explain what the feature does

2. THE Feature_Request_Template SHALL include a section for "Use Case and Motivation" with guidance to explain why the feature is needed

3. THE Feature_Request_Template SHALL include a section for "Proposed Solution" with guidance to describe the implementation approach

4. THE Feature_Request_Template SHALL include a section for "Alternative Approaches" with guidance to discuss other possible solutions

5. THE Feature_Request_Template SHALL include a section for "Additional Context" for links, screenshots, or related discussions

6. THE Feature_Request_Template SHALL include a section for "Related Issues or PRs" to link to existing discussions

7. THE Feature_Request_Template SHALL include guidance about contract security considerations if the feature involves smart contract changes

### Requirement 6: Template Discoverability

**User Story:** As a new contributor, I want to easily find and use the issue templates, so that I can follow the project's contribution guidelines.

#### Acceptance Criteria

1. WHEN a user navigates to the GitHub repository and clicks "New Issue", THE System SHALL display both bug report and feature request templates as options, provided the `.github/ISSUE_TEMPLATE/` directory and template files are properly configured

2. WHEN a user selects a template, THE System SHALL pre-populate the issue form with the template content

3. WHEN a user views the repository's CONTRIBUTING.md, THE System SHALL reference the issue templates and explain when to use each one

4. THE System SHALL ensure templates are discoverable without requiring users to navigate to `.github/ISSUE_TEMPLATE/` manually

### Requirement 7: Template Consistency with Project Standards

**User Story:** As a maintainer, I want issue templates to align with project standards, so that reported issues are consistent with the project's development workflow.

#### Acceptance Criteria

1. THE Bug_Report_Template SHALL reference the project's [Error Message Style Guide](ERROR_STYLE_GUIDE.md) and ask reporters to include exact error messages

2. THE Bug_Report_Template SHALL reference the project's [Architecture Documentation](ARCHITECTURE.md) and ask reporters to identify which component is affected

3. THE Bug_Report_Template SHALL reference the project's [Events Documentation](EVENTS.md) if the bug involves event emission

4. THE Feature_Request_Template SHALL reference the project's [Contributing Guide](CONTRIBUTING.md) and link to the development setup instructions

5. THE Feature_Request_Template SHALL reference the project's [Architecture Documentation](ARCHITECTURE.md) to help requesters understand the system design

6. BOTH templates SHALL include links to the project's security policy and ask reporters to follow responsible disclosure for security issues

### Requirement 8: Environment Information Capture

**User Story:** As a developer, I want to capture detailed environment information in bug reports, so that I can identify environment-specific issues.

#### Acceptance Criteria

1. THE Bug_Report_Template SHALL include a section for "Environment Details" with fields for:
   - Rust version (output of `rustc --version`)
   - Stellar CLI version (output of `stellar --version`)
   - Node.js version (output of `node --version`)
   - Operating system and version
   - Soroban_Network (devnet/testnet/mainnet)

2. THE Bug_Report_Template SHALL include instructions to run `cargo --version` and paste the output

3. THE Bug_Report_Template SHALL include instructions to run `uname -a` (or equivalent for Windows) and paste the output

4. THE Bug_Report_Template SHALL include a note explaining that environment information helps identify version-specific bugs

### Requirement 9: Test Reproducibility

**User Story:** As a developer, I want bug reports to include reproducible test commands, so that I can quickly verify and debug the issue.

#### Acceptance Criteria

1. THE Bug_Report_Template SHALL include a section for "Test Command Output" with instructions to:
   - Run the failing test or command
   - Paste the full output (including error messages and stack traces)
   - Include the exact command used

2. THE Bug_Report_Template SHALL include examples of common test commands:
   - `cargo test --verbose`
   - `stellar contract invoke --id <CONTRACT_ID> -- <function> <args>`
   - `npm test`

3. THE Bug_Report_Template SHALL include a note explaining that reproducible test cases are essential for debugging

4. THE Bug_Report_Template SHALL include instructions to use code blocks (triple backticks) for formatting command output

### Requirement 10: Security Issue Handling

**User Story:** As a maintainer, I want to ensure security issues are reported responsibly, so that vulnerabilities are disclosed safely.

#### Acceptance Criteria

1. BOTH templates SHALL include a note directing security issues to the project's [Security Policy](SECURITY.md)

2. BOTH templates SHALL include a link to the responsible disclosure process

3. THE Bug_Report_Template SHALL include a checkbox asking "Is this a security issue?" with instructions to report via the security policy instead

4. THE System SHALL ensure that security-related issues are not publicly disclosed before a fix is available

### Requirement 11: Template Validation and Completeness

**User Story:** As a maintainer, I want to ensure issues are complete before they are submitted, so that I can reduce back-and-forth clarification.

#### Acceptance Criteria

1. THE Bug_Report_Template SHALL include a checklist of required fields that reporters should complete before submitting

2. THE Feature_Request_Template SHALL include a checklist of recommended fields that requesters should complete before submitting

3. THE Bug_Report_Template SHALL include a note explaining that incomplete issues may be closed and asked to be resubmitted

4. THE System SHALL display the templates in a way that encourages users to fill out all sections

### Requirement 12: Template Maintenance and Updates

**User Story:** As a maintainer, I want to easily update issue templates, so that they remain relevant as the project evolves.

#### Acceptance Criteria

1. THE System SHALL store templates as markdown files in `.github/ISSUE_TEMPLATE/` for easy version control

2. THE System SHALL allow templates to be updated via pull requests like any other project file

3. THE System SHALL include comments in template files explaining the purpose of each section

4. THE System SHALL document the template structure in the project's contributing guide

### Requirement 13: Contract-Specific Information

**User Story:** As a Bug_Reporter, I want to provide contract-specific information, so that developers can quickly identify which contract is affected.

#### Acceptance Criteria

1. THE Bug_Report_Template SHALL include a dedicated field for Contract_ID with:
   - An explanation of what a Contract_ID is
   - An example format
   - Instructions to find the Contract_ID (e.g., from deployment logs or StellarExpert)

2. THE Bug_Report_Template SHALL include a field for the contract function name (e.g., `deposit`, `withdraw`, `rebalance`)

3. THE Bug_Report_Template SHALL include a field for the contract parameters or arguments used

4. THE Bug_Report_Template SHALL include a field for the transaction hash (if applicable) for on-chain issues

### Requirement 14: Network-Specific Guidance

**User Story:** As a Bug_Reporter, I want clear guidance about network-specific issues, so that I can provide the correct network context.

#### Acceptance Criteria

1. THE Bug_Report_Template SHALL include a section explaining the three Soroban_Networks:
   - Devnet: Local development network
   - Testnet: Public test network for integration testing
   - Mainnet: Production network

2. THE Bug_Report_Template SHALL include guidance to identify which network the issue occurs on

3. THE Bug_Report_Template SHALL include a note explaining that network-specific issues (e.g., testnet-only bugs) are important to identify

4. THE Feature_Request_Template SHALL include guidance about network-specific considerations for proposed features

### Requirement 15: Integration with CONTRIBUTING.md

**User Story:** As a contributor, I want the CONTRIBUTING.md to reference issue templates, so that I know how to report issues correctly.

#### Acceptance Criteria

1. THE System SHALL update CONTRIBUTING.md to include a section on "Reporting Issues"

2. THE System SHALL explain when to use the bug report template vs. the feature request template

3. THE System SHALL include links to the issue templates in `.github/ISSUE_TEMPLATE/`

4. THE System SHALL reference the security policy for security-related issues

5. THE System SHALL include guidance on issue labels and how they are used in the project

