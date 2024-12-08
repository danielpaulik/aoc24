fn input() -> &'static str {
    include_str!("../inputs/1.txt")
}

type OrderingRules = [Vec<u8>; 100];
type PageList = Vec<u8>;

fn read_rules_and_process_lists<F>(mut predicate: F)
where F: FnMut(&OrderingRules, PageList) {
    enum ReadPhase {
        OrderingRules,
        PageLists,
    }

    let mut phase = ReadPhase::OrderingRules;
    let mut ordering_rules: OrderingRules = [const { Vec::new() }; 100];

    input().lines().for_each(|line| {
        if line.is_empty() {
            phase = ReadPhase::PageLists;
            return
        }

        match phase {
            ReadPhase::OrderingRules => {
                let mut split = line.split("|");
                let left = split.next().unwrap().parse::<u8>().unwrap();
                let right = split.next().unwrap().parse::<u8>().unwrap();
                ordering_rules[left as usize].push(right);
            }
            ReadPhase::PageLists => {
                predicate(&ordering_rules, line.split(',').map(|value| value.parse::<u8>().unwrap()).collect());
            }
        }
    });
}

fn get_correct_pair_order(ordering_rules: &OrderingRules, a: u8, b: u8) -> std::cmp::Ordering {
    let a_should_precede_b = ordering_rules[a as usize].contains(&b);
    let b_should_precede_a = ordering_rules[b as usize].contains(&a);

    if a_should_precede_b && b_should_precede_a {
        panic!("Inconsistent ordering rules");
    }

    if a_should_precede_b {
        std::cmp::Ordering::Less
    } else if b_should_precede_a {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

fn is_page_list_correctly_ordered(page_list: &PageList, ordering_rules: &OrderingRules) -> bool {
    page_list.windows(2).all(|window| {
        get_correct_pair_order(ordering_rules, window[0], window[1]) == std::cmp::Ordering::Less
    })
}

fn day5_1() -> u32 {
    let mut sum_of_correctly_ordered_middle_values = 0;

    read_rules_and_process_lists(|ordering_rules, page_list| {
        if is_page_list_correctly_ordered(&page_list, ordering_rules) {
            sum_of_correctly_ordered_middle_values += page_list[page_list.len() / 2] as u32;
        }
    });

    sum_of_correctly_ordered_middle_values
}

fn day5_2() -> u32 {
    let mut sum_of_reordered_middle_values = 0;

    read_rules_and_process_lists(|ordering_rules, mut page_list| {
        if is_page_list_correctly_ordered(&page_list, ordering_rules) {
            return;
        }

        page_list.sort_unstable_by(|a, b| get_correct_pair_order(ordering_rules, *a, *b));

        sum_of_reordered_middle_values += page_list[page_list.len() / 2] as u32;
    });

    sum_of_reordered_middle_values
}

fn main() {
    assert_eq!(day5_1(), 5108);
    assert_eq!(day5_2(), 7380);
}
