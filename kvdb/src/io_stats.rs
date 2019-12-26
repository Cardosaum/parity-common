// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Generic statistics for key-value databases

/// Statistic for the `span` period
pub struct IoStats {
	/// Number of transaction.
	pub transactions: u64,
	/// Number of read operations.
	pub reads: u64,
	/// Number of reads resulted in a read from cache.
	pub cache_reads: u64,
	/// Number of write operations.
	pub writes: u64,
	/// Start of the statistic period.
	pub start: std::time::Instant,
	/// Total duration of the statistic period.
	pub span: std::time::Duration,
}

impl IoStats {
	/// Empty statistic report.
	pub fn empty() -> Self {
		Self {
			transactions: 0,
			reads: 0,
			cache_reads: 0,
			writes: 0,
			start: std::time::Instant::now(),
			span: std::time::Duration::default(),
		}
	}

	/// Average batch (transaction) size (writes per transaction)
	pub fn avg_batch_size(&self) -> f64 {
		if self.writes == 0 { return 0.0 }
		self.transactions as f64 / self.writes as f64
	}

	/// Read operations per second.
	pub fn reads_per_sec(&self) -> f64 {
		if self.span.as_secs_f64() == 0.0 { return 0.0 }

		self.reads as f64 / self.span.as_secs_f64()
	}

	/// Write operations per second.
	pub fn writes_per_sec(&self) -> f64 {
		if self.span.as_secs_f64() == 0.0 { return 0.0 }

		self.writes as f64 / self.span.as_secs_f64()
	}

	/// Total number of operations per second.
	pub fn ops_per_sec(&self) -> f64 {
		if self.span.as_secs_f64() == 0.0 { return 0.0 }

		(self.writes as f64 + self.reads as f64) / self.span.as_secs_f64()
	}

	/// Transactions per second.
	pub fn transactions_per_sec(&self) -> f64 {
		if self.span.as_secs_f64() == 0.0 { return 0.0 }

		(self.transactions as f64) / self.span.as_secs_f64()
	}

	pub fn cache_hit_ration(&self) -> f64 {
		if self.reads == 0 { return 0.0 }

		self.cache_reads as f64 / self.reads as f64
	}
 }