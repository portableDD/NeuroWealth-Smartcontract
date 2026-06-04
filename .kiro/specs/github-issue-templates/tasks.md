# Implementation Plan: GitHub Issue Templates

## Overview

This implementation plan breaks down the creation of GitHub issue templates into discrete, actionable tasks. The feature consists of creating two standardized markdown templates (bug report and feature request), a GitHub configuration file, updating the CONTRIBUTING.md guide, and verifying that templates are discoverable and functional in GitHub's UI.

The implementation follows a logical progression: first setting up the directory structure and creating the bug report template, then the feature request template, then the configuration file, then updating CONTRIBUTING.md, and finally verifying everything works correctly.

## Tasks

- [x] 1. Create `.github/ISSUE_TEMPLATE/` directory structure
  - Create the `.github/ISSUE_TEMPLATE/` directory if it doesn't exist
  - Verify the directory is properly created and accessible
  - _Requirements: 3.1_

- [x] 2. Create bug report template file
  - [x] 2.1 Create `.github/ISSUE_TEMPLATE/bug_report.md` with YAML front matter
    - Add YAML front matter with name, about, title, and labels
    - Set title prefix to "Bug: "
    - Set labels to ["bug"]
    - _Requirements: 1.1, 3.2, 4.1_
  
  - [x] 2.2 Add Description section to bug report template
    - Include guidance text explaining what to describe
    - _Requirements: 1.1, 4.3_
  
  - [x] 2.3 Add Soroban Network section to bug report template
    - Include checkboxes for devnet, testnet, mainnet
    - Include explanation of each network
    - Include guidance about why network context matters
    - _Requirements: 1.1, 14.1, 14.2_
  
  - [x] 2.4 Add Contract Information section to bug report template
    - Add Contract ID field with example format and instructions
    - Add Contract Function field for affected function name
    - Add Contract Parameters field for arguments used
    - Include guidance text for each field
    - _Requirements: 1.1, 1.4, 4.2, 13.1, 13.2, 13.3_
  
  - [x] 2.5 Add Steps to Reproduce section to bug report template
    - Include numbered list format with guidance
    - Include example structure showing how to format steps
    - _Requirements: 1.1, 4.6_
  
  - [x] 2.6 Add Expected Behavior and Actual Behavior sections to bug report template
    - Create separate sections for clarity
    - Include guidance text for each
    - _Requirements: 1.1, 4.7_
  
  - [x] 2.7 Add Test Command Output section to bug report template
    - Include instructions to run failing test and paste output
    - Include common test command examples (cargo test, stellar contract invoke, npm test)
    - Include guidance about using code blocks for formatting
    - _Requirements: 1.1, 1.5, 4.3, 9.1, 9.2, 9.3, 9.4_
  
  - [x] 2.8 Add Environment Details section to bug report template
    - Include fields for Rust version, Stellar CLI version, Node.js version, Cargo version
    - Include fields for OS and architecture
    - Include field for Soroban network and RPC endpoint
    - Include guidance about why environment info matters
    - _Requirements: 1.1, 8.1, 8.2, 8.3, 8.4_
  
  - [x] 2.9 Add Error Message section to bug report template
    - Include field for exact error message
    - Reference ERROR_STYLE_GUIDE.md
    - Include guidance about error categories
    - _Requirements: 1.1, 7.1_
  
  - [x] 2.10 Add Verification Checklist to bug report template
    - Include checkboxes for cargo fmt, cargo clippy, full test suite
    - Include checkboxes for CONTRIBUTING.md review and security verification
    - Include note about incomplete issues being closed
    - _Requirements: 1.1, 4.5, 11.1, 11.3_
  
  - [x] 2.11 Add Additional Context section to bug report template
    - Include guidance for screenshots, links, related discussions
    - _Requirements: 1.1, 4.3_
  
  - [x] 2.12 Add Transaction Hash section to bug report template
    - Include field for on-chain transaction hash
    - Include guidance about when this is applicable
    - _Requirements: 1.1, 13.4_
  
  - [x] 2.13 Add Security Issue Notice to bug report template
    - Include prominent warning about security issues
    - Include link to SECURITY.md
    - Include checkbox for "Is this a security issue?"
    - _Requirements: 1.1, 10.1, 10.2, 10.3_

- [x] 3. Create feature request template file
  - [x] 3.1 Create `.github/ISSUE_TEMPLATE/feature_request.md` with YAML front matter
    - Add YAML front matter with name, about, title, and labels
    - Set title prefix to "Feature: "
    - Set labels to ["enhancement"]
    - _Requirements: 2.1, 3.3, 5.1_
  
  - [x] 3.2 Add Feature Description section to feature request template
    - Include guidance text explaining what to describe
    - _Requirements: 2.1, 5.1_
  
  - [x] 3.3 Add Use Case and Motivation section to feature request template
    - Include guidance text explaining why feature is needed
    - Include example showing how to articulate value
    - _Requirements: 2.1, 5.2_
  
  - [x] 3.4 Add Proposed Solution section to feature request template
    - Include guidance text for implementation approach
    - Include examples of what to include (architecture, pseudocode, data structures)
    - _Requirements: 2.1, 5.3_
  
  - [x] 3.5 Add Alternative Approaches section to feature request template
    - Include guidance text for discussing trade-offs
    - Include example showing how to compare approaches
    - _Requirements: 2.1, 5.4_
  
  - [x] 3.6 Add Security Considerations section to feature request template
    - Include guidance about smart contract security implications
    - Include considerations for access control, state changes, attack vectors, error handling
    - Reference SECURITY.md, ERROR_STYLE_GUIDE.md, and ARCHITECTURE.md
    - _Requirements: 2.1, 5.7, 7.1, 7.6_
  
  - [x] 3.7 Add Network-Specific Considerations section to feature request template
    - Include guidance about devnet vs testnet vs mainnet
    - Include example showing network-specific considerations
    - _Requirements: 2.1, 5.5, 14.4_
  
  - [x] 3.8 Add Architecture Alignment section to feature request template
    - Include guidance about aligning with project architecture
    - Reference ARCHITECTURE.md
    - _Requirements: 2.1, 7.5_
  
  - [x] 3.9 Add Additional Context section to feature request template
    - Include guidance for links, screenshots, related discussions
    - _Requirements: 2.1, 5.6_
  
  - [x] 3.10 Add Related Issues or PRs section to feature request template
    - Include fields for linking to related issues and PRs
    - Include guidance about issue relationships (Closes, Related to, Depends on)
    - _Requirements: 2.1, 5.6_
  
  - [x] 3.11 Add Completeness Checklist to feature request template
    - Include checkboxes for CONTRIBUTING.md review, ARCHITECTURE.md review, security considerations
    - Include note about more complete requests being more likely to be implemented
    - _Requirements: 2.1, 11.2_

- [ ] 4. Create template configuration file
  - [ ] 4.1 Create `.github/ISSUE_TEMPLATE/config.yml` with GitHub configuration
    - Set blank_issues_enabled to false
    - Add contact_links for Security Issues, Documentation, and Architecture
    - Include proper URLs to SECURITY.md, CONTRIBUTING.md, and ARCHITECTURE.md
    - _Requirements: 3.4, 3.5_
  
  - [~] 4.2 Verify config.yml is valid YAML
    - Ensure proper indentation and syntax
    - Verify all required fields are present
    - _Requirements: 3.4_

- [ ] 5. Update CONTRIBUTING.md with issue template guidance
  - [~] 5.1 Add "Reporting Issues" section to CONTRIBUTING.md
    - Create new section after existing content
    - _Requirements: 15.1_
  
  - [~] 5.2 Add Bug Report guidance to CONTRIBUTING.md
    - Explain when to use bug report template
    - Include link to bug_report.md template
    - List conditions for using bug report template
    - _Requirements: 15.2, 6.3_
  
  - [~] 5.3 Add Feature Request guidance to CONTRIBUTING.md
    - Explain when to use feature request template
    - Include link to feature_request.md template
    - List conditions for using feature request template
    - _Requirements: 15.2, 6.3_
  
  - [~] 5.4 Add Security Issue guidance to CONTRIBUTING.md
    - Reference SECURITY.md for security-related issues
    - Explain responsible disclosure process
    - _Requirements: 15.4, 10.1, 10.2_
  
  - [~] 5.5 Add Issue Labels section to CONTRIBUTING.md
    - Document common issue labels (bug, enhancement, documentation, good first issue, security, help wanted)
    - Explain how labels are used in the project
    - _Requirements: 15.5_

- [ ] 6. Verify template files are properly formatted
  - [~] 6.1 Verify bug_report.md is valid markdown
    - Check heading structure is correct
    - Verify code blocks are properly formatted
    - Verify links are valid
    - Verify checkboxes are properly formatted
    - _Requirements: 1.2, 3.2_
  
  - [~] 6.2 Verify feature_request.md is valid markdown
    - Check heading structure is correct
    - Verify code blocks are properly formatted
    - Verify links are valid
    - Verify checkboxes are properly formatted
    - _Requirements: 2.2, 3.3_
  
  - [~] 6.3 Verify all template links reference correct files
    - Check SECURITY.md links are correct
    - Check CONTRIBUTING.md links are correct
    - Check ERROR_STYLE_GUIDE.md links are correct
    - Check ARCHITECTURE.md links are correct
    - Check EVENTS.md links are correct
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 7.6_

- [ ] 7. Verify templates are discoverable in GitHub UI
  - [~] 7.1 Verify `.github/ISSUE_TEMPLATE/` directory exists in repository
    - Confirm directory is visible in GitHub web interface
    - _Requirements: 3.1, 6.1_
  
  - [~] 7.2 Verify bug_report.md is discoverable
    - Navigate to GitHub repository
    - Click "New Issue"
    - Verify bug report template appears as an option
    - Verify template name and description are correct
    - _Requirements: 3.2, 6.1, 6.2_
  
  - [~] 7.3 Verify feature_request.md is discoverable
    - Navigate to GitHub repository
    - Click "New Issue"
    - Verify feature request template appears as an option
    - Verify template name and description are correct
    - _Requirements: 3.3, 6.1, 6.2_
  
  - [~] 7.4 Verify config.yml contact links are displayed
    - Click "New Issue"
    - Verify contact links appear (Security Issues, Documentation, Architecture)
    - Verify links are clickable and point to correct URLs
    - _Requirements: 3.4, 3.5_

- [ ] 8. Verify template functionality in GitHub UI
  - [~] 8.1 Verify bug report template pre-populates correctly
    - Select bug report template
    - Verify title is pre-filled with "Bug: "
    - Verify all sections are present in the form
    - Verify guidance text is visible
    - Verify checkboxes are functional
    - _Requirements: 1.1, 1.3, 6.2_
  
  - [~] 8.2 Verify feature request template pre-populates correctly
    - Select feature request template
    - Verify title is pre-filled with "Feature: "
    - Verify all sections are present in the form
    - Verify guidance text is visible
    - Verify checkboxes are functional
    - _Requirements: 2.1, 2.3, 6.2_
  
  - [~] 8.3 Verify bug report template labels are auto-applied
    - Create a test issue using bug report template
    - Verify "bug" label is automatically applied
    - _Requirements: 3.2_
  
  - [~] 8.4 Verify feature request template labels are auto-applied
    - Create a test issue using feature request template
    - Verify "enhancement" label is automatically applied
    - _Requirements: 3.3_

- [ ] 9. Verify markdown rendering in GitHub issues
  - [~] 9.1 Verify code blocks render correctly in bug report
    - Create test issue with code block examples
    - Verify code blocks display with syntax highlighting
    - _Requirements: 1.2_
  
  - [~] 9.2 Verify code blocks render correctly in feature request
    - Create test issue with code block examples
    - Verify code blocks display with syntax highlighting
    - _Requirements: 2.2_
  
  - [~] 9.3 Verify links render correctly in both templates
    - Create test issues with template links
    - Verify all links are clickable and functional
    - _Requirements: 1.2, 2.2_

- [ ] 10. Final verification and cleanup
  - [~] 10.1 Verify all template files are committed to repository
    - Confirm `.github/ISSUE_TEMPLATE/bug_report.md` is in Git
    - Confirm `.github/ISSUE_TEMPLATE/feature_request.md` is in Git
    - Confirm `.github/ISSUE_TEMPLATE/config.yml` is in Git
    - Confirm CONTRIBUTING.md updates are in Git
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 12.1_
  
  - [~] 10.2 Verify CONTRIBUTING.md is properly updated
    - Confirm "Reporting Issues" section is present
    - Confirm all guidance is clear and complete
    - Confirm links to templates are correct
    - _Requirements: 15.1, 15.2, 15.3, 15.4, 15.5_
  
  - [~] 10.3 Verify templates are accessible to new contributors
    - Test that templates appear for users not logged in
    - Test that templates appear for new contributors
    - Verify templates are easy to discover
    - _Requirements: 6.1, 6.2, 6.3, 6.4_

## Notes

- All template files use GitHub-flavored markdown syntax
- Templates are stored in version control and can be updated via pull requests
- GitHub automatically detects the `.github/ISSUE_TEMPLATE/` directory and makes templates available
- Templates include guidance text to help contributors provide complete information
- Templates reference existing project documentation (SECURITY.md, CONTRIBUTING.md, ERROR_STYLE_GUIDE.md, ARCHITECTURE.md, EVENTS.md)
- The config.yml file disables blank issues and provides quick access to important resources
- Manual testing in GitHub UI is essential to verify templates display and function correctly
- All requirements are covered by implementation tasks
