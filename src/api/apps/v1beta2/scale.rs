// Generated from definition io.k8s.api.apps.v1beta2.Scale

/// Scale represents a scaling request for a resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scale {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
    pub metadata: Option<::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    pub spec: Option<::v1_12::api::apps::v1beta2::ScaleSpec>,

    /// current status of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status. Read-only.
    pub status: Option<::v1_12::api::apps::v1beta2::ScaleStatus>,
}

// Begin apps/v1beta2/Scale

// Generated from operation patchAppsV1beta2NamespacedDeploymentScale

impl Scale {
    /// partially update scale of the specified Deployment
    ///
    /// Use [`PatchAppsV1beta2NamespacedDeploymentScaleResponse`](./enum.PatchAppsV1beta2NamespacedDeploymentScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `dry_run`
    ///
    ///     When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_apps_v1beta2_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        body: &::v1_12::apimachinery::pkg::apis::meta::v1::Patch,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/deployments/{name}/scale?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Scale::patch_apps_v1beta2_namespaced_deployment_scale`](./struct.Scale.html#method.patch_apps_v1beta2_namespaced_deployment_scale)
#[derive(Debug)]
pub enum PatchAppsV1beta2NamespacedDeploymentScaleResponse {
    Ok(::v1_12::api::apps::v1beta2::Scale),
    Unauthorized,
    Other,
}

impl ::Response for PatchAppsV1beta2NamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchAppsV1beta2NamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchAppsV1beta2NamespacedDeploymentScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchAppsV1beta2NamespacedDeploymentScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAppsV1beta2NamespacedReplicaSetScale

impl Scale {
    /// partially update scale of the specified ReplicaSet
    ///
    /// Use [`PatchAppsV1beta2NamespacedReplicaSetScaleResponse`](./enum.PatchAppsV1beta2NamespacedReplicaSetScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `dry_run`
    ///
    ///     When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_apps_v1beta2_namespaced_replica_set_scale(
        name: &str,
        namespace: &str,
        body: &::v1_12::apimachinery::pkg::apis::meta::v1::Patch,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}/scale?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Scale::patch_apps_v1beta2_namespaced_replica_set_scale`](./struct.Scale.html#method.patch_apps_v1beta2_namespaced_replica_set_scale)
#[derive(Debug)]
pub enum PatchAppsV1beta2NamespacedReplicaSetScaleResponse {
    Ok(::v1_12::api::apps::v1beta2::Scale),
    Unauthorized,
    Other,
}

impl ::Response for PatchAppsV1beta2NamespacedReplicaSetScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchAppsV1beta2NamespacedReplicaSetScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchAppsV1beta2NamespacedReplicaSetScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchAppsV1beta2NamespacedReplicaSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAppsV1beta2NamespacedStatefulSetScale

impl Scale {
    /// partially update scale of the specified StatefulSet
    ///
    /// Use [`PatchAppsV1beta2NamespacedStatefulSetScaleResponse`](./enum.PatchAppsV1beta2NamespacedStatefulSetScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `dry_run`
    ///
    ///     When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_apps_v1beta2_namespaced_stateful_set_scale(
        name: &str,
        namespace: &str,
        body: &::v1_12::apimachinery::pkg::apis::meta::v1::Patch,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/statefulsets/{name}/scale?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Scale::patch_apps_v1beta2_namespaced_stateful_set_scale`](./struct.Scale.html#method.patch_apps_v1beta2_namespaced_stateful_set_scale)
#[derive(Debug)]
pub enum PatchAppsV1beta2NamespacedStatefulSetScaleResponse {
    Ok(::v1_12::api::apps::v1beta2::Scale),
    Unauthorized,
    Other,
}

impl ::Response for PatchAppsV1beta2NamespacedStatefulSetScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchAppsV1beta2NamespacedStatefulSetScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchAppsV1beta2NamespacedStatefulSetScaleResponse::Unauthorized, 0)),
            _ => Ok((PatchAppsV1beta2NamespacedStatefulSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readAppsV1beta2NamespacedDeploymentScale

impl Scale {
    /// read scale of the specified Deployment
    ///
    /// Use [`ReadAppsV1beta2NamespacedDeploymentScaleResponse`](./enum.ReadAppsV1beta2NamespacedDeploymentScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_apps_v1beta2_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/deployments/{name}/scale?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Scale::read_apps_v1beta2_namespaced_deployment_scale`](./struct.Scale.html#method.read_apps_v1beta2_namespaced_deployment_scale)
#[derive(Debug)]
pub enum ReadAppsV1beta2NamespacedDeploymentScaleResponse {
    Ok(::v1_12::api::apps::v1beta2::Scale),
    Unauthorized,
    Other,
}

impl ::Response for ReadAppsV1beta2NamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadAppsV1beta2NamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadAppsV1beta2NamespacedDeploymentScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadAppsV1beta2NamespacedDeploymentScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readAppsV1beta2NamespacedReplicaSetScale

impl Scale {
    /// read scale of the specified ReplicaSet
    ///
    /// Use [`ReadAppsV1beta2NamespacedReplicaSetScaleResponse`](./enum.ReadAppsV1beta2NamespacedReplicaSetScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_apps_v1beta2_namespaced_replica_set_scale(
        name: &str,
        namespace: &str,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}/scale?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Scale::read_apps_v1beta2_namespaced_replica_set_scale`](./struct.Scale.html#method.read_apps_v1beta2_namespaced_replica_set_scale)
#[derive(Debug)]
pub enum ReadAppsV1beta2NamespacedReplicaSetScaleResponse {
    Ok(::v1_12::api::apps::v1beta2::Scale),
    Unauthorized,
    Other,
}

impl ::Response for ReadAppsV1beta2NamespacedReplicaSetScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadAppsV1beta2NamespacedReplicaSetScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadAppsV1beta2NamespacedReplicaSetScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadAppsV1beta2NamespacedReplicaSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation readAppsV1beta2NamespacedStatefulSetScale

impl Scale {
    /// read scale of the specified StatefulSet
    ///
    /// Use [`ReadAppsV1beta2NamespacedStatefulSetScaleResponse`](./enum.ReadAppsV1beta2NamespacedStatefulSetScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_apps_v1beta2_namespaced_stateful_set_scale(
        name: &str,
        namespace: &str,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/statefulsets/{name}/scale?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Scale::read_apps_v1beta2_namespaced_stateful_set_scale`](./struct.Scale.html#method.read_apps_v1beta2_namespaced_stateful_set_scale)
#[derive(Debug)]
pub enum ReadAppsV1beta2NamespacedStatefulSetScaleResponse {
    Ok(::v1_12::api::apps::v1beta2::Scale),
    Unauthorized,
    Other,
}

impl ::Response for ReadAppsV1beta2NamespacedStatefulSetScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadAppsV1beta2NamespacedStatefulSetScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadAppsV1beta2NamespacedStatefulSetScaleResponse::Unauthorized, 0)),
            _ => Ok((ReadAppsV1beta2NamespacedStatefulSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAppsV1beta2NamespacedDeploymentScale

impl Scale {
    /// replace scale of the specified Deployment
    ///
    /// Use [`ReplaceAppsV1beta2NamespacedDeploymentScaleResponse`](./enum.ReplaceAppsV1beta2NamespacedDeploymentScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `dry_run`
    ///
    ///     When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_apps_v1beta2_namespaced_deployment_scale(
        name: &str,
        namespace: &str,
        body: &::v1_12::api::apps::v1beta2::Scale,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/deployments/{name}/scale?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Scale::replace_apps_v1beta2_namespaced_deployment_scale`](./struct.Scale.html#method.replace_apps_v1beta2_namespaced_deployment_scale)
#[derive(Debug)]
pub enum ReplaceAppsV1beta2NamespacedDeploymentScaleResponse {
    Ok(::v1_12::api::apps::v1beta2::Scale),
    Created(::v1_12::api::apps::v1beta2::Scale),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceAppsV1beta2NamespacedDeploymentScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAppsV1beta2NamespacedDeploymentScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAppsV1beta2NamespacedDeploymentScaleResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceAppsV1beta2NamespacedDeploymentScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAppsV1beta2NamespacedDeploymentScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAppsV1beta2NamespacedReplicaSetScale

impl Scale {
    /// replace scale of the specified ReplicaSet
    ///
    /// Use [`ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse`](./enum.ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `dry_run`
    ///
    ///     When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_apps_v1beta2_namespaced_replica_set_scale(
        name: &str,
        namespace: &str,
        body: &::v1_12::api::apps::v1beta2::Scale,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}/scale?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Scale::replace_apps_v1beta2_namespaced_replica_set_scale`](./struct.Scale.html#method.replace_apps_v1beta2_namespaced_replica_set_scale)
#[derive(Debug)]
pub enum ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse {
    Ok(::v1_12::api::apps::v1beta2::Scale),
    Created(::v1_12::api::apps::v1beta2::Scale),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAppsV1beta2NamespacedStatefulSetScale

impl Scale {
    /// replace scale of the specified StatefulSet
    ///
    /// Use [`ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse`](./enum.ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Scale
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `dry_run`
    ///
    ///     When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_apps_v1beta2_namespaced_stateful_set_scale(
        name: &str,
        namespace: &str,
        body: &::v1_12::api::apps::v1beta2::Scale,
        dry_run: Option<&str>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/statefulsets/{name}/scale?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`Scale::replace_apps_v1beta2_namespaced_stateful_set_scale`](./struct.Scale.html#method.replace_apps_v1beta2_namespaced_stateful_set_scale)
#[derive(Debug)]
pub enum ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse {
    Ok(::v1_12::api::apps::v1beta2::Scale),
    Created(::v1_12::api::apps::v1beta2::Scale),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse::Created(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse::Other, 0)),
        }
    }
}

// End apps/v1beta2/Scale

impl ::Resource for Scale {
    fn api_version() -> &'static str {
        "apps/v1beta2"
    }

    fn group() -> &'static str {
        "apps"
    }

    fn kind() -> &'static str {
        "Scale"
    }

    fn version() -> &'static str {
        "v1beta2"
    }
}

impl<'de> ::serde::Deserialize<'de> for Scale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Scale;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Scale")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<::v1_12::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_12::api::apps::v1beta2::ScaleSpec> = None;
                let mut value_status: Option<::v1_12::api::apps::v1beta2::ScaleStatus> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = ::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as ::Resource>::api_version() {
                                return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as ::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = ::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as ::Resource>::kind() {
                                return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&value_kind), &<Self::Value as ::Resource>::kind()));
                            }
                        },
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Scale {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "Scale",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
                "status",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for Scale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Scale",
            0 +
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as ::Resource>::api_version())?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as ::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.spec {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
