Feature: List yaks
  Displays all yaks with their completion status and hierarchy

  Scenario: List a single yak
    Given I have a clean git repository
    And I have added the yak "Fix the bug"
    When I run "yx list"
    Then the output should be:
      """
        ○ Fix the bug
      """
