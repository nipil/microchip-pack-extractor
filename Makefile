all: clean flamegraph.svg flamechart.svg

filtered.folded: tracing.folded
	sed -E 's/^ThreadId\([0-9]+\); ?// ; /^ThreadId\([0-9]+\) [0-9]+$$/d' $+ > $@

flamegraph.svg: filtered.folded
	inferno-flamegraph $+ > $@

flamechart.svg: filtered.folded
	inferno-flamegraph --flamechart $+ > $@

clean:
	rm -f filtered.folded flamegraph.svg flamechart.svg
