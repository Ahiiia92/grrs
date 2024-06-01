use grrs::find_matches;
use grrs::Cli;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use assert_fs::prelude::*; // dealing with temporary files

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matches() {
        let args = Cli {
            pattern: "foo".to_string(),
            path: std::path::PathBuf::from("test.txt"),
        };
        let mut result = Vec::new();
        find_matches(&args, &mut result).unwrap();
        let output = String::from_utf8(result).unwrap();
        assert_eq!(output, "foo: 10\n");
    }

    #[test]
    fn test_file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("grrs")?;
        cmd.arg("foobar").arg("test/file/doesnt/exist.txt");
        cmd.assert().failure().stderr(predicate::str::contains("No such file or directory"));
        Ok(())
    }

    #[test]
    fn test_write_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("sample.txt")?;
        file.write_str("A test\nActual content\nMore content\nAnother test\n")?;

        let mut cmd = Command::cargo_bin("grrs")?;
        cmd.arg("test").arg(file.path());
        cmd.assert().success().stdout("A test\nAnother test\n");
        Ok(())
    }

    #[test]
    fn test_pattern_not_found() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("grrs")?;
        cmd.arg("").arg("test.txt");
        cmd.assert().failure().stderr(predicate::str::contains("No pattern provided"));
        Ok(())
    }
}
