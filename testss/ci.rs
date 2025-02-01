use gh_workflows::*;

#[test]
fn main() {
    // Create a basic workflow
    let workflow = Workflow::setup_rust();

    // Generate the ci.yml
    workflow.generate().unwrap();
}
