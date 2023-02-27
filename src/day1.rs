pub fn solve(input: &str) -> (u32, u32) {
    let numbers = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|number| number.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .fold([0; 3], |mut array, number| {
            if number > array[0] {
                array[0] = number
            } else if number > array[1] {
                array[1] = number;
            } else if number > array[2] {
                array[2] = number;
            }
            array
        });

    (numbers[0], numbers[0] + numbers[1] + numbers[2])
}
