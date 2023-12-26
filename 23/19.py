from common import standard_inputs


inputs = standard_inputs(19, True, True)
test = [
'px{a<2006:qkq,m>2090:A,rfg}',
'pv{a>1716:R,A}',
'lnx{m>1548:A,A}',
'rfg{s<537:gd,x>2440:R,A}',
'qs{s>3448:A,lnx}',
'qkq{x<1416:A,crn}',
'crn{x>2662:A,R}',
'in{s<1351:px,qqz}',
'qqz{s>2770:qs,m<1801:hdj,R}',
'gd{a>3333:R,R}',
'hdj{m>838:A,pv}',
'',
'{x=787,m=2655,a=1222,s=2876}',
'{x=1679,m=44,a=2067,s=496}',
'{x=2036,m=264,a=79,s=2244}',
'{x=2461,m=1339,a=466,s=291}',
'{x=2127,m=1623,a=2188,s=1013}'
]


def process_inputs(inputs):
    workflows = {}
    parts = []
    is_parts = False
    for line in inputs:
        if not line:
            is_parts = True
        elif not is_parts:
            workflow_name, workflow = line.replace('}', '').split('{')
            rules = []
            for rule in workflow.split(','):
                if ':' in rule:
                    rules.append(rule.split(':'))
                else:
                    rules.append(('True', rule))
            workflows[workflow_name] = rules
        else:
            part = eval(line.replace('{', '{\'').replace('=', ', \':').replace(',', ',\''))
            parts.append(part)
    return workflows, parts


def part_one(inputs):
    workflows, parts = process_inputs(inputs)
    print(workflows, parts)


def part_two(inputs):
    pass


if __name__ == '__main__':
    assert part_one(test) == Y
    print(part_one(inputs))
    assert part_two(test) == Z
    print(part_two(inputs))
