#[macro_export]
macro_rules! ensure {
    // Cas de base avec tous les paramètres
    ($cond:expr, $msg:expr, $status:expr) => {
        ensure!($cond, crate::error::Error::Generic($msg, $status));
    };

    ($cond:expr) => {
        ensure!($cond, crate::error::Error::Internal);
    };

    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };

    ($cond:expr, $msg:expr) => {
        if !$cond {
            return Err(crate::error::Error::Generic($msg, crate::error::StatusCode::INTERNAL_SERVER_ERROR));
        }
    };

    ($cond:expr, $msg:expr, $status:expr) => {
        if !$cond {
            return Err(crate::error::Error::Generic($msg, $status));
        }

    };
}

/* #[cfg(test)]
mod tests {
    use crate::error::Error;
    use axum::http::StatusCode;

    #[test]
    fn test_ensure_zero_args() {
        fn callback(state: bool) -> Result<(), Error> {
            ensure!(state);
            Ok(())
        }

        assert!(callback(true).is_ok());

        assert!(callback(false).is_err());

        let result_error = callback(false).unwrap_err();
        assert_eq!(result_error.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(result_error.error, crate::error::Error::Internal);
    }

    #[test]
    fn test_ensure_one_args_error() {
        fn callback(state: bool, error: Error) -> Result<(), ApiError> {
            ensure!(state, error);
            Ok(())
        }

        assert!(callback(true, Error::Internal).is_ok());
        assert!(callback(true, Error::NotFound("test")).is_ok());

        assert!(callback(false, Error::NotFound("test")).is_err());

        let result_error = callback(false, Error::NotFound("test")).unwrap_err();
        assert_eq!(result_error.status(), StatusCode::NOT_FOUND);
        assert_eq!(result_error.error, crate::error::Error::NotFound("test"));
    }

    #[test]
    fn test_ensure_one_args_str() {}
}
 */
