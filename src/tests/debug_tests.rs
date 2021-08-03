use std::io::Read;

use cfg_if::cfg_if;
use gag::BufferRedirect;

use crate::*;

const EXPECTED_BLANK: &str = "";

#[test]
fn debug_code_block() {
    expect! { EXPECTED: i32 => 0, 42 }
    actual! { actual: i32 }

    // subject under test
    debug! {{ actual = life_universe_everything(); }}

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_code_block_if_err_result() {
    expect! { EXPECTED: &str => "", "it's broke: true" }
    actual! { actual = String::new() }

    // subject under test
    debug! { its_broke() => ERR result { actual = format!("it's broke: {}", result) } }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_code_block_if_err_result_discard_err() {
    expect! { EXPECTED: &str => "", "it's broke" }
    actual! { actual = "" }

    // subject under test
    debug! { its_broke() => _ERR { actual = "it's broke" } }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_code_block_if_ok_result() {
    expect! { EXPECTED: &str => "", "not broke: 42" }
    actual! { actual = String::new() }

    // subject under test
    debug! { it_aint_broke() => OK result { actual = format!("not broke: {}", result) } }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_code_block_if_ok_result_discard_ok() {
    expect! { EXPECTED: &str => "", "not broke" }
    actual! { actual = "" }

    // subject under test
    debug! { it_aint_broke() => _OK { actual = "not broke" } }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_code_blocks_if_err_result() {
    expect! { EXPECTED: i32 => 0, -42 }
    actual! { actual: i32 }

    // subject under test
    debug! {
        is_it_broke(false) =>
            OK value { actual = value }
            ERR err { actual = err }
    }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_code_blocks_if_err_result_discard_err() {
    expect! { EXPECTED: i32 => 0, -42 }
    actual! { actual: i32 }

    // subject under test
    debug! {
        is_it_broke(false) =>
            OK value { actual = value }
            _ERR { actual = -42 }
    }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_code_blocks_if_err_result_discard_ok() {
    expect! { EXPECTED: i32 => 0, -42 }
    actual! { actual: i32 }

    // subject under test - discard ok
    debug! {
        is_it_broke(false) =>
            _OK { actual = life_universe_everything() }
            ERR err { actual = err }
    }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_code_blocks_if_err_result_discard_results() {
    expect! { EXPECTED: i32 => 0, -42 }
    actual! { actual: i32 }

    // subject under test - discard ok
    debug! {
        is_it_broke(false) =>
            _OK { actual = life_universe_everything() }
            _ERR { actual = -42 }
    }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_code_blocks_if_ok_result() {
    expect! { EXPECTED: i32 => 0, 42 }
    actual! { actual: i32 }

    // subject under test, use err
    debug! {
        is_it_broke(true) =>
            OK value { actual = value }
            ERR err { actual = err }
    }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_code_blocks_if_ok_result_discard_err() {
    expect! { EXPECTED: i32 => 0, 42 }
    actual! { actual: i32 }

    // subject under test
    debug! {
        is_it_broke(true) =>
            OK value { actual = value }
            _ERR { actual = -42 }
    }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_code_blocks_if_ok_result_discard_ok() {
    expect! { EXPECTED: i32 => 0, 42 }
    actual! { actual: i32 }

    // subject under test, use err
    debug! {
        is_it_broke(true) =>
            _OK { actual = life_universe_everything() }
            ERR err { actual = err }
    }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_code_blocks_if_ok_result_discard_results() {
    expect! { EXPECTED: i32 => 0, 42 }
    actual! { actual: i32 }

    // subject under test
    debug! {
        is_it_broke(true) =>
            _OK { actual = life_universe_everything() }
            _ERR { actual = -42 }
    }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn debug_message_formatted_error_if_err_result() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only; -42: -42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                OK "debug message only";
                ERR "debug error message only; {}", -42;
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_error_if_err_result_discard_err() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only; -42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                OK "debug message only";
                _ERR "debug error message only; {}", -42;
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_error_if_err_result_discard_ok() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only; -42: -42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                _OK "debug message only";
                ERR "debug error message only; {}", -42;
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_error_if_err_result_discard_results() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only; -42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                _OK "debug message only";
                _ERR "debug error message only; {}", -42;
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_error_if_ok_result() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only: 42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                OK "debug message only";
                ERR "debug error message only; {}", -42;
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_error_if_ok_result_discard_err() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only: 42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                OK "debug message only";
                _ERR "debug error message only; {}", -42;
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_error_if_ok_result_discard_ok() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                _OK "debug message only";
                ERR "debug error message only; {}", -42;
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_error_if_ok_result_discard_results() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                _OK "debug message only";
                _ERR "debug error message only; {}", -42;
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_if_err_result() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only; -42: -42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                OK "debug message only";
                ERR "debug error message only; {}", -42;
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_if_err_result_discard_err() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                OK "debug message only";
                _ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_if_err_result_discard_ok() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only: -42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                _OK "debug message only";
                ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_if_err_result_discard_results() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                _OK "debug message only";
                _ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_if_ok_result() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only: 42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                OK "debug message only";
                ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_if_ok_result_discard_err() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only: 42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                OK "debug message only";
                _ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_if_ok_result_discard_ok() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                _OK "debug message only";
                ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_if_ok_result_discard_results() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                _OK "debug message only";
                _ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_success_if_err_result() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only; -42: -42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                OK "debug message only; {}", 42;
                ERR "debug error message only; {}", -42;
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_success_if_err_result_discard_err() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                OK "debug message only; {}", 42;
                _ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_success_if_err_result_discard_ok() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only: -42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                _OK "debug message only; {}", 42;
                ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_success_if_err_result_discard_results() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                _OK "debug message only; {}", 42;
                _ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_success_if_ok_result() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only; 42: 42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                OK "debug message only; {}", 42;
                ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_success_if_ok_result_discard_err() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only; 42: 42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                OK "debug message only; {}", 42;
                _ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_success_if_ok_result_discard_ok() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only; 42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                _OK "debug message only; {}", 42;
                ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_formatted_success_if_ok_result_discard_results() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only; 42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                _OK "debug message only; {}", 42;
                _ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_if_err_result() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only: -42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                OK "debug message only";
                ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_if_err_result_discard_err() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                OK "debug message only";
                _ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_if_err_result_discard_ok() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only: -42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                _OK "debug message only";
                ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_if_err_result_discard_results() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(false) =>
                _OK "debug message only";
                _ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_message_if_ok_result() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only: 42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                OK "debug message only";
                ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_if_ok_result_discard_err() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only: 42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                OK "debug message only";
                _ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_if_ok_result_discard_ok() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                _OK "debug message only";
                ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_message_if_ok_result_discard_results() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! {
            is_it_broke(true) =>
                _OK "debug message only";
                _ERR "debug error message only";
        }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_stderr() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! { ERR "debug error message only" }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_stderr_formatted() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only - 42; \"so long and thanks for all the fish!\"\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! { ERR "debug error message only - {}; {:?}", 42, hh_guide() }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_stderr_formatted_if_err_result() {
    expect! {
        EXPECTED: &str
            => "", "ERROR: debug error message only - 42; \"so long and thanks for all the fish!\": true\n"
    }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! { its_broke() => ERR "debug error message only - {}; {:?}", 42, hh_guide() }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_stderr_formatted_if_err_result_discard_err() {
    expect! {
        EXPECTED: &str
            => "", "ERROR: debug error message only - 42; \"so long and thanks for all the fish!\"\n"
    }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! { its_broke() => _ERR "debug error message only - {}; {:?}", 42, hh_guide() }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_stderr_if_err_result() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only: true\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! { its_broke() => ERR "debug error message only" }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_stderr_if_err_result_discard_err() {
    expect! { EXPECTED: &str => "", "ERROR: debug error message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! { its_broke() => _ERR "debug error message only" }
    };

    assert_eq!(EXPECTED, actual_err);
    assert_eq!(EXPECTED_BLANK, actual_out, "alternate io expected to be blank");
}

#[test]
fn debug_stdout() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! { "debug message only" }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_stdout_formatted() {
    expect! {
        EXPECTED: &str
            => "", "DEBUG: debug message only - 42; \"so long and thanks for all the fish!\"\n"
    }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! { "debug message only - {}; {:?}", 42, hh_guide() }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_stdout_formatted_if_ok_result() {
    expect! {
        EXPECTED: &str
            => "", "DEBUG: debug message only - 42; \"so long and thanks for all the fish!\": 42\n"
    }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! { it_aint_broke() => OK "debug message only - {}; {:?}", 42, hh_guide() }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_stdout_formatted_if_ok_result_discard_ok() {
    expect! {
        EXPECTED: &str
            => "", "DEBUG: debug message only - 42; \"so long and thanks for all the fish!\"\n"
    }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! { it_aint_broke() => _OK "debug message only - {}; {:?}", 42, hh_guide() }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_stdout_if_ok_result() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only: 42\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! { it_aint_broke() => OK "debug message only" }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[test]
fn debug_stdout_if_ok_result_discard_ok() {
    expect! { EXPECTED: &str => "", "DEBUG: debug message only\n" }

    let (actual_out, actual_err) = capture! {
        // subject under test
        debug! { it_aint_broke() => _OK "debug message only" }
    };

    assert_eq!(EXPECTED, actual_out);
    assert_eq!(EXPECTED_BLANK, actual_err, "alternate io expected to be blank");
}

#[cfg(debug_assertions)]
const fn life_universe_everything() -> i32 { 42 }

#[cfg(debug_assertions)]
fn is_it_broke(you_tell_me: bool) -> Result<i32, i32> { if you_tell_me { Ok(42) } else { Err(-42) } }

#[cfg(debug_assertions)]
const fn it_aint_broke() -> Result<i32, bool> { Ok(42) }

#[cfg(debug_assertions)]
const fn its_broke() -> Result<i32, bool> { Err(true) }

#[cfg(debug_assertions)]
const fn hh_guide() -> &'static str { "so long and thanks for all the fish!" }
