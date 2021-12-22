use crate::interfaces::ClientLike;
use crate::utils;

#[cfg(feature = "metrics")]
use crate::modules::metrics::Stats;

/// Functions that implement the internal metrics interface, largely controlled by the `metrics` feature flag.
pub trait MetricsInterface: ClientLike + Sized {
  /// Read the number of buffered commands that have not yet been sent to the server.
  fn command_queue_len(&self) -> usize {
    utils::read_atomic(&self.inner().cmd_buffer_len)
  }

  /// Read latency metrics across all commands.
  ///
  /// This metric reflects the total latency experienced by callers, including time spent waiting in memory to be written and network latency.
  /// Features such as automatic reconnect, `reconnect-on-auth-error`, and frame serialization time can all affect these values.
  #[cfg(feature = "metrics")]
  #[cfg_attr(docsrs, doc(cfg(feature = "metrics")))]
  fn read_latency_metrics(&self) -> Stats {
    self.inner().latency_stats.read().read_metrics()
  }

  /// Read and consume latency metrics, resetting their values afterwards.
  #[cfg(feature = "metrics")]
  #[cfg_attr(docsrs, doc(cfg(feature = "metrics")))]
  fn take_latency_metrics(&self) -> Stats {
    self.inner().latency_stats.write().take_metrics()
  }

  /// Read network latency metrics across all commands.
  ///
  /// This metric only reflects time spent waiting on a response. It will factor in reconnect time if a response doesn't arrive due to a connection
  /// closing, but it does not factor in the time a command spends waiting to be written, serialization time, backpressure, etc.  
  #[cfg(feature = "metrics")]
  #[cfg_attr(docsrs, doc(cfg(feature = "metrics")))]
  fn read_network_latency_metrics(&self) -> Stats {
    self.inner().network_latency_stats.read().read_metrics()
  }

  /// Read and consume network latency metrics, resetting their values afterwards.
  #[cfg(feature = "metrics")]
  #[cfg_attr(docsrs, doc(cfg(feature = "metrics")))]
  fn take_network_latency_metrics(&self) -> Stats {
    self.inner().network_latency_stats.write().take_metrics()
  }

  /// Read request payload size metrics across all commands.
  #[cfg(feature = "metrics")]
  #[cfg_attr(docsrs, doc(cfg(feature = "metrics")))]
  fn read_req_size_metrics(&self) -> Stats {
    self.inner().req_size_stats.read().read_metrics()
  }

  /// Read and consume request payload size metrics, resetting their values afterwards.
  #[cfg(feature = "metrics")]
  #[cfg_attr(docsrs, doc(cfg(feature = "metrics")))]
  fn take_req_size_metrics(&self) -> Stats {
    self.inner().req_size_stats.write().take_metrics()
  }

  /// Read response payload size metrics across all commands.
  #[cfg(feature = "metrics")]
  #[cfg_attr(docsrs, doc(cfg(feature = "metrics")))]
  fn read_res_size_metrics(&self) -> Stats {
    self.inner().res_size_stats.read().read_metrics()
  }

  /// Read and consume response payload size metrics, resetting their values afterwards.
  #[cfg(feature = "metrics")]
  #[cfg_attr(docsrs, doc(cfg(feature = "metrics")))]
  fn take_res_size_metrics(&self) -> Stats {
    self.inner().res_size_stats.write().take_metrics()
  }
}