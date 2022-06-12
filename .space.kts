job("Build and Push: Chaos") {
    docker {
        build {
            context = "."
            file = "./Dockerfile"
            args["HTTP_PROXY"] = "http://0.0.0.0:9999"
            labels["vendor"] = "scattered-systems"
        }

        push("scattered-systems.registry.jetbrains.space/p/scattered-systems/containers/chaos") {
            tags("0.0.\$JB_SPACE_EXECUTION_NUMBER", "latest")
        }
    }
}

