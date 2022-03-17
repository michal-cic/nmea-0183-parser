#[cfg(test)]
mod parser {
    #[test]
    fn parse_static_valid() {
        let parser = nmea_0183_parser::init();
        let slices = parser.parse(
            "$GPRMC,015606.000,A,3150.7584,N,11712.0491,E,0.00,231.36,280715,,,A*67<CR><LF>",
        );
        assert_eq!(slices[0], "$GPRMC");
        assert_eq!(slices[1], "015606.000");
        assert_eq!(slices[2], "A");
        assert_eq!(slices[3], "3150.7584");
        assert_eq!(slices[4], "N");
        assert_eq!(slices[5], "11712.0491");
        assert_eq!(slices[6], "E");
        assert_eq!(slices[7], "0.00");
        assert_eq!(slices[8], "231.36");
        assert_eq!(slices[9], "280715");
        assert_eq!(slices[10], "");
        assert_eq!(slices[11], "");
        assert_eq!(slices[12], "A*67<CR><LF>");
    }

    #[test]
    fn parse_static_invalid() {
        let parser = nmea_0183_parser::init();
        let slices = parser.parse("$GNRMC,094905.103,V,,,,,0.00,0.00,100180,,,N*58<CR><LF>");
        assert_eq!(slices[0], "$GNRMC");
        assert_eq!(slices[1], "094905.103");
        assert_eq!(slices[2], "V");
        assert_eq!(slices[3], "");
        assert_eq!(slices[4], "");
        assert_eq!(slices[5], "");
        assert_eq!(slices[6], "");
        assert_eq!(slices[7], "0.00");
        assert_eq!(slices[8], "0.00");
        assert_eq!(slices[9], "100180");
        assert_eq!(slices[10], "");
        assert_eq!(slices[11], "");
        assert_eq!(slices[12], "N*58<CR><LF>");
    }
}
