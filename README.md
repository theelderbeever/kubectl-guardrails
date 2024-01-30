# kubectl-guardrails

Sometimes I am firing away `kubectl` commands with reckless abandon and sometimes I am using the wrong context and sometimes that wrong context is the production cluster. Hello bad day...

Enter `kubectl guardrails`. The idea behind this is to let developers put guardrails on the contexts and subcommands that they care about and let everything else passthrough unabated. Prompting a yes/no when running `get pods` is burdensome meanwhile blocking `delete <resource>` can be really valuable in preventing accidents.

```console
❯ kubectl delete pods hello
================ Context ==================

Current Context: Prod-Cluster

================ Dry Run ==================

pod "hello" deleted (dry run)

===========================================

Would you like to proceed? (y/N): y

❯ /opt/homebrew/bin/kubectl delete pods hello

pod "hello" deleted
```


## Install

I will be adding pre-build binaries however, right now you need `cargo`.

1. `cargo install --git https://github.com/theelderbeever/kubectl-guardrails.git`
2. Run `which kubectl`
3. Create your `~/.kube/guardrails` file as shown in the Configuration section.
   1. Populate the `bin` key with the result of `which kubectl`
4. (Optional) In your `.bashrc`/`.zshrc` add `alias "kubectl=kubectl guardrails"`

## Configuration

```yaml
bin: /opt/homebrew/bin/kubectl # This was mine on MacOS
contexts:
  - name: Prod-Cluster # This should match a context in your `KUBECONFIG` file
    prompts:
      - name: apply
      - name: delete
        dry-run: true
``````