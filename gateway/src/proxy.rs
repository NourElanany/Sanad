use axum::{
    body::Body,
    http::{HeaderMap, Method, Uri},
    response::Response,
};
use shared::{AppConfig, SanadError, SanadResult};
use reqwest::Client;
use std::collections::HashMap;
use tracing::{error, info, warn};

/// Service registry for managing microservice endpoints
#[derive(Clone)]
pub struct ServiceRegistry {
    services: HashMap<String, ServiceInfo>,
    client: Client,
}

/// Information about a microservice
#[derive(Debug, Clone)]
struct ServiceInfo {
    base_url: String,
    health_endpoint: String,
    timeout_seconds: u64,
}

impl ServiceRegistry {
    /// Create a new service registry
    pub async fn new(config: &AppConfig) -> SanadResult<Self> {
        let mut services = HashMap::new();

        // Register all microservices
        // In production, these would be loaded from service discovery or configuration
        services.insert(
            "quran-service".to_string(),
            ServiceInfo {
                base_url: "http://localhost:8081".to_string(),
                health_endpoint: "/health".to_string(),
                timeout_seconds: 30,
            },
        );

        services.insert(
            "hadith-service".to_string(),
            ServiceInfo {
                base_url: "http://localhost:8082".to_string(),
                health_endpoint: "/health".to_string(),
                timeout_seconds: 30,
            },
        );

        services.insert(
            "stories-service".to_string(),
            ServiceInfo {
                base_url: "http://localhost:8083".to_string(),
                health_endpoint: "/health".to_string(),
                timeout_seconds: 30,
            },
        );

        services.insert(
            "prayer-times-service".to_string(),
            ServiceInfo {
                base_url: "http://localhost:8084".to_string(),
                health_endpoint: "/health".to_string(),
                timeout_seconds: 10,
            },
        );

        services.insert(
            "calendar-service".to_string(),
            ServiceInfo {
                base_url: "http://localhost:8085".to_string(),
                health_endpoint: "/health".to_string(),
                timeout_seconds: 10,
            },
        );

        services.insert(
            "ai-service".to_string(),
            ServiceInfo {
                base_url: "http://localhost:8086".to_string(),
                health_endpoint: "/health".to_string(),
                timeout_seconds: 60, // AI service may take longer
            },
        );

        services.insert(
            "search-service".to_string(),
            ServiceInfo {
                base_url: "http://localhost:8087".to_string(),
                health_endpoint: "/health".to_string(),
                timeout_seconds: 30,
            },
        );

        services.insert(
            "audio-analysis-service".to_string(),
            ServiceInfo {
                base_url: "http://localhost:8088".to_string(),
                health_endpoint: "/health".to_string(),
                timeout_seconds: 120, // Audio processing may take longer
            },
        );

        services.insert(
            "khatma-service".to_string(),
            ServiceInfo {
                base_url: "http://localhost:8089".to_string(),
                health_endpoint: "/health".to_string(),
                timeout_seconds: 30,
            },
        );

        services.insert(
            "notification-service".to_string(),
            ServiceInfo {
                base_url: "http://localhost:8090".to_string(),
                health_endpoint: "/health".to_string(),
                timeout_seconds: 10,
            },
        );

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.server.request_timeout_seconds))
            .build()
            .map_err(|e| SanadError::Internal(format!("Failed to create HTTP client: {}", e)))?;

        let registry = Self { services, client };

        // Perform initial health checks
        registry.check_all_services().await;

        Ok(registry)
    }

    /// Proxy a request to the appropriate microservice
    pub async fn proxy_request(
        &self,
        service_name: &str,
        uri: Uri,
        method: Method,
        headers: HeaderMap,
        body: Body,
    ) -> Result<Response, SanadError> {
        let service = self
            .services
            .get(service_name)
            .ok_or_else(|| SanadError::NotFound(format!("Service '{}' not found", service_name)))?;

        // Extract the path and query from the original URI
        let path_and_query = uri.path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("/");

        // Remove the service prefix from the path
        let cleaned_path = path_and_query
            .strip_prefix(&format!("/api/v1/{}", service_name.replace("-service", "")))
            .unwrap_or(path_and_query);

        // Construct the target URL
        let target_url = format!("{}{}", service.base_url, cleaned_path);

        info!("Proxying {} {} to {}", method, path_and_query, target_url);

        // Convert axum body to bytes
        let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Failed to read request body: {}", e);
                return Err(SanadError::Internal("Failed to read request body".to_string()));
            }
        };

        // Build the request
        let mut request_builder = self.client.request(method, &target_url);

        // Copy headers (excluding host and content-length which will be set automatically)
        for (name, value) in headers.iter() {
            if name != "host" && name != "content-length" {
                request_builder = request_builder.header(name, value);
            }
        }

        // Add body if present
        if !body_bytes.is_empty() {
            request_builder = request_builder.body(body_bytes);
        }

        // Set timeout
        request_builder = request_builder.timeout(std::time::Duration::from_secs(service.timeout_seconds));

        // Execute the request
        let response = request_builder
            .send()
            .await
            .map_err(|e| {
                error!("Failed to proxy request to {}: {}", service_name, e);
                if e.is_timeout() {
                    SanadError::ServiceUnavailable(format!("Service '{}' timeout", service_name))
                } else if e.is_connect() {
                    SanadError::ServiceUnavailable(format!("Cannot connect to service '{}'", service_name))
                } else {
                    SanadError::ExternalApi {
                        service: service_name.to_string(),
                        message: e.to_string(),
                    }
                }
            })?;

        // Convert reqwest response to axum response
        let mut response_builder = Response::builder().status(response.status());

        // Copy response headers
        for (name, value) in response.headers().iter() {
            response_builder = response_builder.header(name, value);
        }

        let body_bytes = response
            .bytes()
            .await
            .map_err(|e| SanadError::Internal(format!("Failed to read response body: {}", e)))?;

        response_builder
            .body(Body::from(body_bytes))
            .map_err(|e| SanadError::Internal(format!("Failed to build response: {}", e)))
    }

    /// Check health of all registered services
    async fn check_all_services(&self) {
        for (service_name, service_info) in &self.services {
            match self.check_service_health(service_name, service_info).await {
                Ok(true) => info!("Service '{}' is healthy", service_name),
                Ok(false) => warn!("Service '{}' is unhealthy", service_name),
                Err(e) => warn!("Failed to check health of service '{}': {}", service_name, e),
            }
        }
    }

    /// Check health of a specific service
    async fn check_service_health(&self, service_name: &str, service_info: &ServiceInfo) -> SanadResult<bool> {
        let health_url = format!("{}{}", service_info.base_url, service_info.health_endpoint);
        
        let response = self
            .client
            .get(&health_url)
            .timeout(std::time::Duration::from_secs(5)) // Short timeout for health checks
            .send()
            .await
            .map_err(|e| SanadError::ExternalApi {
                service: service_name.to_string(),
                message: e.to_string(),
            })?;

        Ok(response.status().is_success())
    }

    /// Get list of all registered services
    pub fn get_services(&self) -> Vec<String> {
        self.services.keys().cloned().collect()
    }

    /// Check if a service is registered
    pub fn has_service(&self, service_name: &str) -> bool {
        self.services.contains_key(service_name)
    }
}