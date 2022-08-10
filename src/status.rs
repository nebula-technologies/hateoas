#[derive(Serialize, Debug, PartialEq, Default)]
pub struct Status {
    pub(crate) message: Option<String>,
    pub(crate) code: Option<u32>,
    pub(crate) http_status_code: Option<u16>,
    pub(crate) session: Option<uuid::Uuid>,
}

impl Status {
    pub fn new(
        message: Option<&str>,
        code: Option<u32>,
        http_status_code: Option<u16>,
        session: Option<uuid::Uuid>,
    ) -> Self {
        Status {
            message: message.map(|t| t.to_string()),
            code,
            http_status_code,
            session,
        }
    }

    /// ## Getting Message
    /// This is for getting the message field from the status object.
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::new(Some("hello world"), None, None, None);
    ///
    /// assert_eq!(status.get_message(), &Some("hello world".to_string()));
    /// ```
    pub fn message(&self) -> &Option<String> {
        &self.message
    }

    /// ## Getting Mutable Message
    /// This is for getting the message field from the status object.
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::new(Some("hello world"), None, None, None);
    /// let mut mut_message = status.get_message_mut();
    ///
    /// *mut_message = Some("Hello Space".to_string());
    ///
    /// assert_eq!(status.get_message(), &Some("Hello Space".to_string()));
    /// ```
    pub fn message_mut(&mut self) -> &mut Option<String> {
        &mut self.message
    }

    /// ## Getting code
    /// Getting the internal status code from the stauts object
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::default();
    /// status.code(&100);
    ///
    /// assert_eq!(status.get_code(), &Some(100));
    /// ```
    pub fn code(&self) -> &Option<u32> {
        &self.code
    }

    /// ## Getting Mutable Code
    /// Getting the internal status code from the stauts object as a mutable reference
    /// allowing for modifications to the internal status code.
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::default();
    ///
    /// let mut status_code = status.get_code_mut();
    /// *status_code = Some(200);
    ///
    /// assert_eq!(status.get_code(), &Some(200));
    /// ```
    pub fn code_mut(&mut self) -> &mut Option<u32> {
        &mut self.code
    }

    /// ## Getter for the HTTP status code
    /// This is for getting the http_status_code.
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::default();
    /// status.http_status_code(&200);
    ///
    /// assert_eq!(status.get_http_status_code(), &Some(200));
    /// ```
    pub fn http_status_code(&self) -> &Option<u16> {
        &self.http_status_code
    }

    /// ## Getter for mutable HTTP status code
    /// This is for getting the http_status_code.
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::default();
    /// status.http_status_code(&200);
    ///
    /// assert_eq!(status.get_http_status_code(), &Some(200));
    /// ```
    pub fn http_status_code_mut(&mut self) -> &Option<u16> {
        &mut self.http_status_code
    }

    /// ## Getter for the Session
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::default();
    /// let uuid = uuid::Uuid::new_v4();
    /// status.session(&uuid);
    ///
    /// assert_eq!(status.get_session(), &Some(uuid));
    /// ```
    pub fn session(&self) -> &Option<uuid::Uuid> {
        &self.session
    }

    /// ## Getter for mutable Session id
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::default();
    /// let uuid = uuid::Uuid::new_v4();
    /// let uuid_2 = uuid::Uuid::new_v4();
    /// status.session(&uuid);
    /// let mut mut_session = status.get_session_mut();
    /// *mut_session = Some(uuid_2);
    ///
    /// assert_eq!(status.get_session(), &Some(uuid_2));
    /// ```
    pub fn session_mut(&mut self) -> &mut Option<uuid::Uuid> {
        &mut self.session
    }

    pub fn get(
        &self,
    ) -> (
        &Option<String>,
        &Option<u32>,
        &Option<u16>,
        &Option<uuid::Uuid>,
    ) {
        (
            &self.message,
            &self.code,
            &self.http_status_code,
            &self.session,
        )
    }

    pub fn get_mut(
        &mut self,
    ) -> (
        &mut Option<String>,
        &mut Option<u32>,
        &mut Option<u16>,
        &mut Option<uuid::Uuid>,
    ) {
        (
            &mut self.message,
            &mut self.code,
            &mut self.http_status_code,
            &mut self.session,
        )
    }
}
