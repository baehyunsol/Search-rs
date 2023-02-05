use crate::agent::{Agent, AgentOption};
use crate::file::{file_name, read_bytes, rmdir};

#[test]
fn kzg_test() {
    let mut agent = Agent::init_new("./test_data/kzg".to_string(), AgentOption::default()).unwrap();

    let test_cases = vec![
        ([0, 0, 0, 0, 2, 1, 0, 1, 0, 12], "emerg"),
        ([1, 1, 0, 0, 1, 3, 9, 0, 0, 0], "dark"),
        ([108, 98, 52, 123, 73, 109, 107, 56, 77, 60], " the"),
    ];

    for (counts, keyword) in test_cases.into_iter() {
        test_unit(&mut agent, counts, keyword);
    }

    rmdir("./test_data/kzg/.index");
}

fn test_unit(agent: &mut Agent, counts: [usize; 10], keyword: &str) {

    let mut file_counts = [0; 10];
    let search_result = agent.search(keyword.as_bytes());

    for (file_path, index) in search_result.into_iter() {
        let file_name = &file_name(&file_path).unwrap();

        if file_name == "Ant kingdom" {
            file_counts[0] += 1;
        }

        else if file_name == "Automation" {
            file_counts[1] += 1;
        }

        else if file_name == "Beauty" {
            file_counts[2] += 1;
        }

        else if file_name == "Climate Change" {
            file_counts[3] += 1;
        }

        else if file_name == "Consciousness" {
            file_counts[4] += 1;
        }

        else if file_name == "Dark Forest" {
            file_counts[5] += 1;
        }

        else if file_name == "Deep Sea" {
            file_counts[6] += 1;
        }

        else if file_name == "Dissatisfaction" {
            file_counts[7] += 1;
        }

        else if file_name == "Dyson Sphere" {
            file_counts[8] += 1;
        }

        else if file_name == "Emergence" {
            file_counts[9] += 1;
        }

        else {
            panic!("{}", file_name);
        }

        let actual_data = read_bytes(&file_path).unwrap();

        assert_eq!(&actual_data[index..(index + keyword.len())], keyword.as_bytes());
    }

    assert_eq!(file_counts, counts);
}