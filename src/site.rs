pub enum Site {
    Visited { url: String, children: Vec<Site>, is_error: bool },
    Found { url: String },
}

impl ToString for Site {
    fn to_string(&self) -> String {
        self.to_string_depth(0)
    }
}

impl Site {
    fn to_string_depth(&self, depth: usize) -> String {
        let branch_marker = "|__";

        match self {
            Site::Found { url } => url.to_string(),
            Site::Visited { url, children, is_error: _ } => {
                let next_depth = depth + 1;
                let indentation = "\t".repeat(next_depth).to_string();

                children.iter()
                    .map(|site| site.to_string_depth(next_depth))
                    .fold(url.to_string(), |acc, next| {
                        format!("{}\n{}{}{}", acc, indentation, branch_marker, next)
                    })
            }
        }
    }
}


mod test {
    use crate::site::Site;

    #[test]
    fn site_map_recursive_pretty_print() {
        let site = Site::Visited {
            url: "http://lol.com".to_string(),
            is_error: false,
            children: vec!(
                Site::Found {
                    url: "http://bbc.com".to_string(),
                },
                Site::Visited {
                    url: "http://lol.com/wow".to_string(),
                    is_error: false,
                    children: vec!(
                        Site::Visited {
                            url: "http://lol.com/wow/more".to_string(),
                            is_error: false,
                            children: vec!(),
                        },
                        Site::Found { url: "http://lol.com/nonoono".to_string() }
                    ),
                },
                Site::Visited {
                    url: "http://new.com/wow".to_string(),
                    is_error: false,
                    children: vec!(),
                }
            ),
        };

        let expected_output = concat!(
        "http://lol.com\n",
        "\t|__http://bbc.com\n",
        "\t|__http://lol.com/wow\n",
        "\t\t|__http://lol.com/wow/more\n",
        "\t\t|__http://lol.com/nonoono\n",
        "\t|__http://new.com/wow"
        );

        assert_eq!(expected_output, site.to_string());
    }
}