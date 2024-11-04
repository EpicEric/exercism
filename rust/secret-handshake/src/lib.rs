pub fn actions(n: u8) -> Vec<&'static str> {
    const ACTIONS: &[(u8, &'static str)] = &[
        (0b00001, "wink"),
        (0b00010, "double blink"),
        (0b00100, "close your eyes"),
        (0b01000, "jump"),
    ];
    let filter_function = |(code, action): &(u8, &'static str)| {
        if n & code != 0 {
            Some(*action)
        } else {
            None
        }
    };
    if n & 0b10000 != 0 {
        ACTIONS.iter().rev().filter_map(filter_function).collect()
    } else {
        ACTIONS.iter().filter_map(filter_function).collect()
    }
}
