.PHONY: bench check-bench

bench:
	@echo "Running Soroban gas benchmarks..."
	@cargo test --test benchmarks -- --nocapture | grep "BENCHMARK|" > .bench_results.tmp
	@echo "Function | CPU Instructions | Memory Bytes"
	@echo "--- | --- | ---"
	@awk -F'|' '{printf "%s | %s | %s\n", $$2, $$3, $$4}' .bench_results.tmp
	@rm .bench_results.tmp

generate-baseline:
	@echo "Generating contracts/BENCHMARKS.md..."
	@echo "# Smart Contract Gas Benchmarks\n" > BENCHMARKS.md
	@echo "*Note: These numbers act as the baseline for CI. Do not edit manually unless updating the baseline.*\n" >> BENCHMARKS.md
	@echo "| Function | CPU Instructions | Memory Bytes |" >> BENCHMARKS.md
	@echo "| --- | --- | --- |" >> BENCHMARKS.md
	@cargo test --test benchmarks -- --nocapture | grep "BENCHMARK|" | awk -F'|' '{printf "| %s | %s | %s |\n", $$2, $$3, $$4}' >> BENCHMARKS.md
	@echo "Baseline updated successfully."