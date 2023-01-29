use crate::agent::{Agent, AgentOption};
use crate::file::{file_name, read_bytes, rmdir};

#[test]
fn ipsum_test() {
    let mut agent = Agent::init_new("./test_data/ipsum".to_string(), AgentOption::default()).unwrap();

    let test_cases = vec![
        ([8, 7, 9, 4], "ipsum"),
        ([4, 2, 1, 2], "porta"),
        ([0, 2, 2, 6], "massa"),
        ([3, 0, 0, 2], "auctor"),
        ([1, 0, 2, 5], "hendrerit"),
        ([2, 3, 4, 5], "venenatis"),
    ];

    for (counts, keyword) in test_cases.into_iter() {
        unit_test(&mut agent, counts, keyword);
    }

    rmdir("./test_data/ipsum/.index");
}

fn unit_test(agent: &mut Agent, counts: [usize; 4], keyword: &str) {

    let mut file_counts = [0; 4];
    let search_result = agent.search(keyword.as_bytes());

    for (file_path, index) in search_result.into_iter() {
        let file_name = &file_name(&file_path).unwrap();

        if file_name == "ipsum1" {
            file_counts[0] += 1;
        }

        else if file_name == "ipsum2" {
            file_counts[1] += 1;
        }

        else if file_name == "ipsum3" {
            file_counts[2] += 1;
        }

        else if file_name == "ipsum4" {
            file_counts[3] += 1;
        }

        else {
            panic!();
        }

        let actual_data = read_bytes(&file_path).unwrap();

        assert_eq!(&actual_data[index..(index + keyword.len())], keyword.as_bytes());
    }

    assert_eq!(file_counts, counts);
}