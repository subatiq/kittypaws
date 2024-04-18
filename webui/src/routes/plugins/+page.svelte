<script lang="ts">
let rows = [
    {name: "dropper", repository: "subatiq/kittypaws-dropper"}
]
</script>

<h1 class="uppercase">Plugins</h1>
<hr />
<div class="my-10 flex flex-col gap-10">
<div class="w-96 flex flex-col gap-5">
	<label class="label">
		<span>Github repository name</span>
		<input class="input" type="text" placeholder="subatiq/kittypaws-dropper" />
		<input class="input" type="text" placeholder="master" />
	</label>

	<a href="/" class="btn variant-filled-primary uppercase">Install</a>
</div>
<hr/>
<div class="flex flex-col gap-5">
    <h1 class="">Installed</h1>

<!-- Responsive Container (recommended) -->
<div class="table-container">
	<!-- Native Table Element -->
	<table class="table table-hover">
		<thead>
			<tr>
				<th>Name</th>
				<th>Github Link</th>
				<th></th>
			</tr>
		</thead>
		<tbody>
			{#each rows as row, i}
				<tr>
					<td>{row.name}</td>
					<td>{row.repository}</td>
	<td><a href="#" class="btn-sm variant-filled-warning">Uninstall</a></td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>


</div>
</div>
