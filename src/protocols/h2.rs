// use sdt::error::Error;
// use sdt::time::Duration;

// const DEFAULT_H2_CONN_WINDOW_SIZE: u32 = 65535;
// const DEFAULT_H2_STREAM_WINDOW_SIZE: u32 = 65535;
// const DEFAULT_H2_MAX_DATA_FRAMES: u32 = 1024 * 16; // 16kb
// const DEFAULT_MAX_SEND_BUF_SIZE: usize = 1024 * 400; // 400kb
// const DEFAULT_MAX_HEADERS_LIST_SIZE: u32 = 16 << 20;


// #[derive(Clone, Debug)]
// pub struct Config {
//     // overall allowed size limit of data the server has to buffer. Per request
//     pub h2_conn_window_size: u32,
//     // individual stream data limit for data buffered for an active stream
//     pub h2_stream_window_size: u32,
//     // maximum amount of response body bytes placed into a frame
//     pub h2_max_data_frames: u32,
//     pub h2_max_concurrent_streams: Option<u32>,
//     pub h2_keep_alive_interval: Option<Duration>,
//     pub h2_keep_alive_timeout: Duration,
//     pub max_send_buffer_size: usize,
//     pub max_header_list_size: u32,
// }

// impl Default for Config {
//     fn default() -> Config {
//         Config {
//             h2_conn_window_size: DEFAULT_H2_CONN_WINDOW_SIZE,
//             h2_stream_window_size: DEFAULT_H2_STREAM_WINDOW_SIZE,
//             h2_max_data_frames: DEFAULT_H2_MAX_DATA_FRAMES,
//             h2_max_concurrent_streams: None,
//             h2_keep_alive_interval: None,
//             h2_keep_alive_timeout: Duration::from_secs(20),
//             max_send_buffer_size: DEFAULT_MAX_SEND_BUF_SIZE,
//             max_header_list_size: DEFAULT_MAX_HEADERS_LIST_SIZE,
//         }
//     }
// }