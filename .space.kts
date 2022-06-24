job("Docker: Build and Push") {
    startOn {
        gitPush {
            enabled: true,
            branchFilters {
                +"refs/head/master"
            }
        }
    }

    docker {
        build {
            context = "."
            file = "./Dockerfile"
            args["HTTP_PROXY"] = "http://0.0.0.0:8888"
            labels["vendor"] = "scattered-systems"
        }

        push("scattered-systems.registry.jetbrains.space/p/chaos/containers/chaos") {
            tags("0.0.\$JB_SPACE_EXECUTION_NUMBER", "latest")
        }
    }
}

