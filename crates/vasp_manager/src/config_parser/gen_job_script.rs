use super::super::template::TERA;
use super::config;
use super::config::format_value;
use eyre::Result;
use tera::Context;

impl config::JobConfig {
    pub fn write_job_script(&self) -> Result<()> {
        let vasp_config = &self.toml_contents["vasp"];
        let slurm_config = &self.toml_contents["slurm"];

        let mut context = Context::new();
        context.insert("partition", &format_value(&slurm_config["partition"]));
        context.insert("num_nodes", &format_value(&slurm_config["num_nodes"]));
        context.insert("num_tasks", &format_value(&slurm_config["num_tasks"]));
        context.insert("vasp_version", &format_value(&vasp_config["version"]));
        context.insert("bin", &format_value(&vasp_config["bin"]));

        let rendered = TERA.render("job_script.sh", &context).unwrap();
        let job_script_path = format!("{}/job_script.sh", self.job_dir);
        std::fs::write(job_script_path, rendered)?;
        Ok(())
    }
}
