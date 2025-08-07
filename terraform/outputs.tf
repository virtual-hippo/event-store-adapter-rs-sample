output "projects" {
  value = data.tidbcloud_projects.default_project.items
}

output "project_id" {
  value = data.tidbcloud_projects.default_project.items[0].id
}

