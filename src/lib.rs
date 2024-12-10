#![deny(clippy::all)]

use std::{
  fs::{self, File, FileTimes},
  time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH},
};

#[cfg(target_os = "macos")]
use std::os::macos::fs::FileTimesExt;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct SetFileTimesOptions {
  pub path: String,
  pub btime: Option<String>,
  pub mtime: Option<String>,
  pub atime: Option<String>,
}

#[napi(object)]
pub struct SetFileTimesResult {
  pub error: Option<String>,
}

#[napi(object)]
pub struct FileTimesResult {
  pub error: Option<String>,
  pub btime: Option<String>,
  pub mtime: Option<String>,
  pub atime: Option<String>,
}

#[napi]
pub fn set_file_times(options: SetFileTimesOptions) -> SetFileTimesResult {
  let btime = options.btime;
  let mtime = options.mtime;
  let atime = options.atime;
  let file = File::options().write(true).open(&options.path);

  match file {
    Ok(file) => {
      let mut results = Vec::new();

      if let Some(btime) = btime {
        #[cfg(target_os = "macos")]
        {
          let (created_secs, created_nanos) = js_timestamp_to_rs_timestamp(&btime);
          let times = FileTimes::new().set_created(create_system_time_from_timestamp(
            &created_secs,
            &created_nanos,
          ));

          results.push(file.set_times(times));
        }
      }

      if let Some(mtime) = mtime {
        let (modified_secs, modified_nanos) = js_timestamp_to_rs_timestamp(&mtime);
        let times = FileTimes::new().set_modified(create_system_time_from_timestamp(
          &modified_secs,
          &modified_nanos,
        ));

        results.push(file.set_times(times));
      }

      if let Some(atime) = atime {
        let (accessed_secs, accessed_nanos) = js_timestamp_to_rs_timestamp(&atime);
        let times = FileTimes::new().set_accessed(create_system_time_from_timestamp(
          &accessed_secs,
          &accessed_nanos,
        ));

        results.push(file.set_times(times));
      }

      for result in results {
        match result {
          Err(error) => {
            return SetFileTimesResult {
              error: Some(format!("{}", error)),
            };
          }
          _ => continue,
        };
      }

      SetFileTimesResult { error: None }
    }
    Err(error) => SetFileTimesResult {
      error: Some(format!("{}", error)),
    },
  }
}

#[napi]
pub fn get_file_times(path: String) -> FileTimesResult {
  match fs::metadata(path) {
    Ok(meta) => FileTimesResult {
      error: None,
      btime: Some(
        system_time_to_seconds(meta.created().unwrap())
          .unwrap()
          .to_string(),
      ),
      mtime: Some(
        system_time_to_seconds(meta.modified().unwrap())
          .unwrap()
          .to_string(),
      ),
      atime: Some(
        system_time_to_seconds(meta.accessed().unwrap())
          .unwrap()
          .to_string(),
      ),
    },
    Err(error) => FileTimesResult {
      error: Some(format!("{}", error)),
      btime: None,
      mtime: None,
      atime: None,
    },
  }
}

fn system_time_to_seconds(system_time: SystemTime) -> Result<u64, SystemTimeError> {
  system_time.duration_since(UNIX_EPOCH).map(|d| d.as_secs())
}

fn create_system_time_from_timestamp(seconds: &u64, nanosecons: &u32) -> SystemTime {
  let duration = Duration::new(*seconds, *nanosecons);
  UNIX_EPOCH + duration
}

// 13(len) -> (secs, nanos)
fn js_timestamp_to_rs_timestamp(js_timestamp_str: &str) -> (u64, u32) {
  let len = js_timestamp_str.len();

  if len == 13 {
    let secs = &js_timestamp_str[..10];
    let nanos = &js_timestamp_str[len - 3..];

    return (
      secs.parse().unwrap(),
      nanos.parse::<u32>().unwrap() * 1000000,
    );
  }

  // len == 10
  (js_timestamp_str.parse().unwrap(), 0)
}
