use anyhow::Result;
use std::path::Path;

use crate::cgroups::common;
use oci_spec::{LinuxCpu, LinuxResources};

use super::controller::Controller;

const CGROUP_CPUSET_CPUS: &str = "cpuset.cpus";
const CGROUP_CPUSET_MEMS: &str = "cpuset.mems";

pub struct CpuSet {}

impl Controller for CpuSet {
    fn apply(linux_resources: &LinuxResources, cgroup_path: &Path) -> Result<()> {
        if let Some(cpuset) = &linux_resources.cpu {
            Self::apply(cgroup_path, cpuset)?;
        }

        Ok(())
    }
}

impl CpuSet {
    fn apply(path: &Path, cpuset: &LinuxCpu) -> Result<()> {
        if let Some(cpus) = &cpuset.cpus {
            common::write_cgroup_file(&path.join(CGROUP_CPUSET_CPUS), cpus)?;
        }

        if let Some(mems) = &cpuset.mems {
            common::write_cgroup_file(&path.join(CGROUP_CPUSET_MEMS), mems)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;
    use crate::cgroups::test::{create_temp_dir, set_fixture, LinuxCpuBuilder};

    fn setup(testname: &str, cgroup_file: &str) -> (PathBuf, PathBuf) {
        let tmp = create_temp_dir(testname).expect("create temp directory for test");
        let cgroup_file = set_fixture(&tmp, cgroup_file, "")
            .expect(&format!("set test fixture for {}", cgroup_file));

        (tmp, cgroup_file)
    }

    #[test]
    fn test_set_cpus() {
        // arrange
        let (tmp, cpus) = setup("test_set_cpus", CGROUP_CPUSET_CPUS);
        let cpuset = LinuxCpuBuilder::new().with_cpus("1-3".to_owned()).build();

        // act
        CpuSet::apply(&tmp, &cpuset).expect("apply cpuset");

        // assert
        let content =
            fs::read_to_string(&cpus).expect(&format!("read {} file content", CGROUP_CPUSET_CPUS));
        assert_eq!(content, "1-3");
    }

    #[test]
    fn test_set_mems() {
        // arrange
        let (tmp, mems) = setup("test_set_mems", CGROUP_CPUSET_MEMS);
        let cpuset = LinuxCpuBuilder::new().with_mems("1-3".to_owned()).build();

        // act
        CpuSet::apply(&tmp, &cpuset).expect("apply cpuset");

        // assert
        let content =
            fs::read_to_string(&mems).expect(&format!("read {} file content", CGROUP_CPUSET_MEMS));
        assert_eq!(content, "1-3");
    }
}
