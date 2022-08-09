

#[derive(Serialize, Debug, PartialEq, Default)]
pub struct Status {
    pub(crate) message: Option<String>,
    pub(crate) code: Option<u32>,
    pub(crate) http_status_code: Option<u16>,
    pub(crate) session: Option<uuid::Uuid>,
}

impl Status {
    pub fn new(
        message: Option<String>,
        code: Option<u32>,
        http_status_code: Option<u16>,
        session: Option<uuid::Uuid>,
    ) -> Self {
        Status {
            message,
            code,
            http_status_code,
            session,
        }
    }
    /// Status - Message
    /// This is for setting a message on the status.
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::default();
    /// status.message("hello world");
    ///
    /// assert_eq!(status, Status::new(Some("hello world".to_string()), None, None, None));
    /// ```
    pub fn message(&mut self, message: &str) {
        self.message = Some(message.to_string());
    }

    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::default();
    /// status.code(&100);
    ///
    /// assert_eq!(status, Status::new(None, Some(100), None, None));
    /// ```
    pub fn code(&mut self, code: &u32) {
        self.code = Some(*code);
    }

    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::default();
    /// status.http_status_code(&200);
    ///
    /// assert_eq!(status, Status::new(None, None, Some(200), None));
    /// ```
    pub fn http_status_code(&mut self, http_status_code: &u16) {
        self.http_status_code = Some(*http_status_code);
    }

    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::default();
    /// let uuid = uuid::Uuid::new_v4();
    /// status.session(&uuid);
    ///
    /// assert_eq!(status, Status::new(None, None, None, Some(uuid)));
    /// ```
    pub fn session(&mut self, session: &uuid::Uuid) {
        self.session = Some(*session);
    }
}
