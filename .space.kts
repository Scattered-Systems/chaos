job("Docker: Build and Push Chaos") {
    startOn {
        gitPush {
            branchFilter {
                +"refs/heads/master"
            }
        }
    }

    docker {
        build {
            context = "."
            customPlatform = "linux/arm"
            file ="./Dockerfile"
            labels["vendor"] = "scattered-systems"
        }

        push("scattered-systems.registry.jetbrains.space/p/scsys/containers/chaos") {
            tags("0.0.\$JB_SPACE_EXECUTION_NUMBER", "latest")
        }
    }
}