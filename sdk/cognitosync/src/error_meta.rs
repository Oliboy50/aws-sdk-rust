// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AlreadyStreamedException(crate::error::AlreadyStreamedException),
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    DuplicateRequestException(crate::error::DuplicateRequestException),
    InternalErrorException(crate::error::InternalErrorException),
    InvalidConfigurationException(crate::error::InvalidConfigurationException),
    InvalidLambdaFunctionOutputException(crate::error::InvalidLambdaFunctionOutputException),
    InvalidParameterException(crate::error::InvalidParameterException),
    LambdaThrottledException(crate::error::LambdaThrottledException),
    LimitExceededException(crate::error::LimitExceededException),
    NotAuthorizedException(crate::error::NotAuthorizedException),
    ResourceConflictException(crate::error::ResourceConflictException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    TooManyRequestsException(crate::error::TooManyRequestsException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AlreadyStreamedException(inner) => inner.fmt(f),
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::DuplicateRequestException(inner) => inner.fmt(f),
            Error::InternalErrorException(inner) => inner.fmt(f),
            Error::InvalidConfigurationException(inner) => inner.fmt(f),
            Error::InvalidLambdaFunctionOutputException(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::LambdaThrottledException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::NotAuthorizedException(inner) => inner.fmt(f),
            Error::ResourceConflictException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::TooManyRequestsException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::BulkPublishError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::BulkPublishError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::BulkPublishErrorKind::AlreadyStreamedException(inner) => {
                    Error::AlreadyStreamedException(inner)
                }
                crate::error::BulkPublishErrorKind::DuplicateRequestException(inner) => {
                    Error::DuplicateRequestException(inner)
                }
                crate::error::BulkPublishErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::BulkPublishErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::BulkPublishErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::BulkPublishErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::BulkPublishErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteDatasetError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteDatasetError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteDatasetErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::DeleteDatasetErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DeleteDatasetErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::DeleteDatasetErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::DeleteDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteDatasetErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DeleteDatasetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeDatasetError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeDatasetError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeDatasetErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::DescribeDatasetErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeDatasetErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::DescribeDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeDatasetErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DescribeDatasetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeIdentityPoolUsageError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeIdentityPoolUsageError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeIdentityPoolUsageErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::DescribeIdentityPoolUsageErrorKind::InvalidParameterException(
                    inner,
                ) => Error::InvalidParameterException(inner),
                crate::error::DescribeIdentityPoolUsageErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::DescribeIdentityPoolUsageErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeIdentityPoolUsageErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::DescribeIdentityPoolUsageErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeIdentityUsageError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeIdentityUsageError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeIdentityUsageErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::DescribeIdentityUsageErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeIdentityUsageErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::DescribeIdentityUsageErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeIdentityUsageErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DescribeIdentityUsageErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetBulkPublishDetailsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetBulkPublishDetailsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetBulkPublishDetailsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::GetBulkPublishDetailsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::GetBulkPublishDetailsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::GetBulkPublishDetailsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetBulkPublishDetailsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetCognitoEventsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetCognitoEventsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetCognitoEventsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::GetCognitoEventsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::GetCognitoEventsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::GetCognitoEventsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetCognitoEventsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::GetCognitoEventsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetIdentityPoolConfigurationError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetIdentityPoolConfigurationError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetIdentityPoolConfigurationErrorKind::InternalErrorException(
                    inner,
                ) => Error::InternalErrorException(inner),
                crate::error::GetIdentityPoolConfigurationErrorKind::InvalidParameterException(
                    inner,
                ) => Error::InvalidParameterException(inner),
                crate::error::GetIdentityPoolConfigurationErrorKind::NotAuthorizedException(
                    inner,
                ) => Error::NotAuthorizedException(inner),
                crate::error::GetIdentityPoolConfigurationErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::GetIdentityPoolConfigurationErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::GetIdentityPoolConfigurationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListDatasetsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListDatasetsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDatasetsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::ListDatasetsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::ListDatasetsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::ListDatasetsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListDatasetsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListIdentityPoolUsageError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListIdentityPoolUsageError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListIdentityPoolUsageErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::ListIdentityPoolUsageErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::ListIdentityPoolUsageErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::ListIdentityPoolUsageErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListIdentityPoolUsageErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListRecordsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListRecordsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListRecordsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::ListRecordsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::ListRecordsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::ListRecordsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListRecordsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::RegisterDeviceError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::RegisterDeviceError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RegisterDeviceErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::RegisterDeviceErrorKind::InvalidConfigurationException(inner) => {
                    Error::InvalidConfigurationException(inner)
                }
                crate::error::RegisterDeviceErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::RegisterDeviceErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::RegisterDeviceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::RegisterDeviceErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::RegisterDeviceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::SetCognitoEventsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::SetCognitoEventsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SetCognitoEventsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::SetCognitoEventsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::SetCognitoEventsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::SetCognitoEventsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::SetCognitoEventsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::SetCognitoEventsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::SetIdentityPoolConfigurationError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::SetIdentityPoolConfigurationError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::SetIdentityPoolConfigurationErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
                crate::error::SetIdentityPoolConfigurationErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
                crate::error::SetIdentityPoolConfigurationErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::SetIdentityPoolConfigurationErrorKind::NotAuthorizedException(inner) => Error::NotAuthorizedException(inner),
                crate::error::SetIdentityPoolConfigurationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::SetIdentityPoolConfigurationErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::SetIdentityPoolConfigurationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::SubscribeToDatasetError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::SubscribeToDatasetError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SubscribeToDatasetErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::SubscribeToDatasetErrorKind::InvalidConfigurationException(inner) => {
                    Error::InvalidConfigurationException(inner)
                }
                crate::error::SubscribeToDatasetErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::SubscribeToDatasetErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::SubscribeToDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::SubscribeToDatasetErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::SubscribeToDatasetErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UnsubscribeFromDatasetError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UnsubscribeFromDatasetError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UnsubscribeFromDatasetErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::UnsubscribeFromDatasetErrorKind::InvalidConfigurationException(
                    inner,
                ) => Error::InvalidConfigurationException(inner),
                crate::error::UnsubscribeFromDatasetErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::UnsubscribeFromDatasetErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::UnsubscribeFromDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UnsubscribeFromDatasetErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::UnsubscribeFromDatasetErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateRecordsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateRecordsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateRecordsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::UpdateRecordsErrorKind::InvalidLambdaFunctionOutputException(
                    inner,
                ) => Error::InvalidLambdaFunctionOutputException(inner),
                crate::error::UpdateRecordsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::UpdateRecordsErrorKind::LambdaThrottledException(inner) => {
                    Error::LambdaThrottledException(inner)
                }
                crate::error::UpdateRecordsErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UpdateRecordsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::UpdateRecordsErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::UpdateRecordsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateRecordsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::UpdateRecordsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
