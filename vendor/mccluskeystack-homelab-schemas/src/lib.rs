// @generated
pub mod homelab {
    pub mod helloworld {
        #[cfg(feature = "homelab-helloworld-v1")]
        // @@protoc_insertion_point(attribute:homelab.helloworld.v1)
        pub mod v1 {
            include!("homelab/helloworld/v1/homelab.helloworld.v1.rs");
            // @@protoc_insertion_point(homelab.helloworld.v1)
        }
    }
    pub mod stats {
        pub mod todos {
            #[cfg(feature = "homelab-stats-todos-v1")]
            // @@protoc_insertion_point(attribute:homelab.stats.todos.v1)
            pub mod v1 {
                include!("homelab/stats/todos/v1/homelab.stats.todos.v1.rs");
                // @@protoc_insertion_point(homelab.stats.todos.v1)
            }
        }
        pub mod workouts {
            #[cfg(feature = "homelab-stats-workouts-v1")]
            // @@protoc_insertion_point(attribute:homelab.stats.workouts.v1)
            pub mod v1 {
                include!("homelab/stats/workouts/v1/homelab.stats.workouts.v1.rs");
                // @@protoc_insertion_point(homelab.stats.workouts.v1)
            }
        }
    }
}