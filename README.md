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

❯ kubectl delete pods hello

pod "hello" deleted
```


## Install

1. Download the latest [release](https://github.com/theelderbeever/kubectl-guardrails/releases) to a place on your path OR `cargo install --git https://github.com/theelderbeever/kubectl-guardrails.git`
2. Create your `~/.kube/guardrails` file as shown in the Configuration section.
3. (Optional) In your `.bashrc`/`.zshrc` add `alias "kubectl=kubectl guardrails"`

## Configuration

```yaml
contexts:
  - name: Prod-Cluster # This should match a context in your `KUBECONFIG` file
    prompts:
      - name: apply
      - name: delete
        dry-run: true
``````