pub fn part_one(input: impl AsRef<str>) -> i64 {
    let input = input.as_ref();
    let mut input = input.chars().peekable();

    let mut res = 0;
    'outer: loop {
        for k in "mul(".chars() {
            match input.next() {
                Some(c) if c == k => {}
                Some(_) => continue 'outer,
                None => break 'outer,
            }
        }

        let mut num1 = 0_u32;
        match input.next_if(|c| c.is_numeric()) {
            Some(c) => {
                num1 = c.to_string().parse::<u32>().unwrap();
            }
            None => continue 'outer,
        }

        while input.peek().is_some() && input.peek().unwrap().is_numeric() {
            num1 *= 10;
            num1 += input.next().unwrap().to_string().parse::<u32>().unwrap();
        }

        match input.next_if_eq(&',') {
            Some(_) => {}
            None => continue 'outer,
        }

        let mut num2 = 0_u32;
        match input.next_if(|c| c.is_numeric()) {
            Some(c) => {
                num2 = c.to_string().parse::<u32>().unwrap();
            }
            None => continue 'outer,
        }

        while input.peek().is_some() && input.peek().unwrap().is_numeric() {
            num2 *= 10;
            num2 += input.next().unwrap().to_string().parse::<u32>().unwrap();
        }

        match input.next_if_eq(&')') {
            Some(_) => {}
            None => continue 'outer,
        }

        res += num1 * num2;
    }

    res as i64
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let input = input.as_ref();
    let mut input = input.chars().peekable();
    let mut res = 0;
    let mut do_mul = true;

    'outer: loop {
        match input.peek() {
            Some(c) if c == &'d' => {
                for k in "do".chars() {
                    match input.next_if_eq(&k) {
                        Some(_) => {}
                        None => continue 'outer,
                    }
                }

                match input.peek() {
                    Some(c) if c == &'(' => {
                        for k in "()".chars() {
                            match input.next_if_eq(&k) {
                                Some(_) => {}
                                None => continue 'outer,
                            }
                        }

                        do_mul = true;
                    }
                    Some(c) if c == &'n' => {
                        for k in "n't()".chars() {
                            match input.next_if_eq(&k) {
                                Some(_) => {}
                                None => continue 'outer,
                            }
                        }

                        do_mul = false;
                    }
                    _ => {}
                }
            }
            Some(c) if c == &'m' => {
                for k in "mul(".chars() {
                    match input.next_if_eq(&k) {
                        Some(_) => {}
                        None => continue 'outer,
                    }
                }

                let mut num1 = 0_u32;
                match input.next_if(|c| c.is_numeric()) {
                    Some(c) => {
                        num1 = c.to_string().parse::<u32>().unwrap();
                    }
                    None => continue 'outer,
                }

                while input.peek().is_some() && input.peek().unwrap().is_numeric() {
                    num1 *= 10;
                    num1 += input.next().unwrap().to_string().parse::<u32>().unwrap();
                }

                match input.next_if_eq(&',') {
                    Some(_) => {}
                    None => continue 'outer,
                }

                let mut num2 = 0_u32;
                match input.next_if(|c| c.is_numeric()) {
                    Some(c) => {
                        num2 = c.to_string().parse::<u32>().unwrap();
                    }
                    None => continue 'outer,
                }

                while input.peek().is_some() && input.peek().unwrap().is_numeric() {
                    num2 *= 10;
                    num2 += input.next().unwrap().to_string().parse::<u32>().unwrap();
                }

                match input.next_if_eq(&')') {
                    Some(_) => {}
                    None => continue 'outer,
                }

                if do_mul {
                    res += num1 * num2;
                }
            }
            Some(_) => {
                input.next();
            }
            None => break,
        }
    }

    res as i64
}
