#[cfg(test)]
mod request_params_test {
    use super::super::request_params::*;

    #[test]
    fn test_param_parsing_full_url() {
        let url = "/foo/bar?p1=f&p2=bär&p3=&p4";
        let rp = RequestParams::from_request_url(url);
        assert_eq!(rp.get("p1",""), "f");
        assert_eq!(rp.get("p2",""), "bär");
        assert_eq!(rp.get("p3","foo"), "");
        assert_eq!(rp.get("p4","foo"), "");
        assert_eq!(rp.get("p5","foo"), "foo");
    }

    #[test]
    fn test_param_parsing_params_2() {
        let url = "?p1=f&p2=bär&p3=&p4";
        let rp = RequestParams::from_request_url(url);
        assert_eq!(rp.get("p1",""), "f");
        assert_eq!(rp.get("p2",""), "bär");
        assert_eq!(rp.get("p3","foo"), "");
        assert_eq!(rp.get("p4","foo"), "");
        assert_eq!(rp.get("p5","foo"), "foo");
    }

    #[test]
    fn test_param_parsing_params_only() {
        let url = "p1=f&p2=bär&p3=&p4";
        let rp = RequestParams::from_request_url(url);
        assert_eq!(rp.get("p1",""), "f");
        assert_eq!(rp.get("p2",""), "bär");
        assert_eq!(rp.get("p3","foo"), "");
        assert_eq!(rp.get("p4","foo"), "");
        assert_eq!(rp.get("p5","foo"), "foo");
    }

    #[test]
    fn test_param_parsing_wrong() {
        let url = "?&&&";
        let rp = RequestParams::from_request_url(url);
        assert_eq!(rp.get("p1","foo"), "foo");
    }

    #[test]
    fn test_get_i64() {
        let url = "/foo/bar?p1=42&p2=-3&p3=33.5&p4=0&p5=null";
        let rp = RequestParams::from_request_url(url);
        assert_eq!(rp.get_i64("p1"), Some(42));
        assert_eq!(rp.get_i64("p2"), Some(-3));
        assert_eq!(rp.get_i64("p3"), None, "Value is not an integer", );
        assert_eq!(rp.get_i64("p4"), Some(0));
        assert_eq!(rp.get_i64("p5"), None);
        assert_eq!(rp.get_i64("p6"), None);
    }
}
