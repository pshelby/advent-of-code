"""
Calculate how many total questions members of groups answered yes.
"""

import logging

FILENAME = "./questions.txt"
LOG_LEVEL = logging.INFO

logging.basicConfig(level=LOG_LEVEL)


def parse_file_into_questions(filename: str) -> list:
    """
    Return list of lists of questions answered.
    """
    parser_logger = logging.getLogger('parser')

    all_groups_answers = []
    groups_answers = []

    with open(filename) as question_fh:
        for line in question_fh.readlines():
            line = line.strip()
            parser_logger.debug('line: "%s"', line)

            if not line:
                all_groups_answers.append(groups_answers)
                parser_logger.debug('all_groups_answers: "%s"', all_groups_answers)
                groups_answers = []
                continue

            groups_answers.append(line)
            parser_logger.debug('groups_answers: "%s"', groups_answers)

        if groups_answers:
            all_groups_answers.append(groups_answers)

    return all_groups_answers


def find_group_common_questions(group_answers: list) -> int:
    """
    Return number of common questions for the group.
    """
    grouper_logger = logging.getLogger('grouper')

    yes_qs = {}
    common_qs = 0

    for indv_answers in group_answers:
        for answer in indv_answers:
            if answer not in yes_qs:
                yes_qs[answer] = 0
            yes_qs[answer] += 1

            if yes_qs[answer] == len(group_answers):
                common_qs += 1

    grouper_logger.debug('yes_qs: "%s"', yes_qs)
    grouper_logger.debug('common_qs: "%d"', common_qs)
    return common_qs


if __name__ == '__main__':
    TOTAL_COMMON_QUESTIONS = 0

    for group_ans in parse_file_into_questions(FILENAME):
        TOTAL_COMMON_QUESTIONS += find_group_common_questions(group_ans)

    logging.info('Common "yes" questions: %d', TOTAL_COMMON_QUESTIONS)
