# CLI Inspirado no Lethal Company — Features

## Escopo do projeto

Desenvolver um terminal inspirado no sistema do jogo Lethal Company.

A proposta é que o usuário “interaja” com uma empresa fictícia através de comandos textuais.

---

# MVP — funcionalidades mínimas

## 1. Boot do terminal
Ao iniciar:
- splash screen
- nome da corporação
- versão do terminal
- mensagem inicial

---

## 2. Prompt interativo
O sistema deve:
- aceitar comandos continuamente
- exibir resposta para cada comando
- manter sessão ativa até sair

---

## 3. Sistema de comandos
Necessário:
- reconhecimento de comandos válidos
- tratamento de comandos inválidos
- sistema de ajuda/listagem

---

## 4. Sistema de planetas
O terminal precisa possuir:
- múltiplos planetas/lua
- informações individuais
- níveis de perigo
- clima
- custo de viagem
- status do planeta

---

## 5. Escaneamento
Feature de:
- scan de planeta
- relatório textual
- geração de informações variáveis
- possibilidade de erro/interferência

---

## 6. Loja (store)
O terminal precisa:
- listar itens
- exibir preços
- permitir compra
- impedir compra sem créditos

Itens possíveis:
- lanterna
- pá
- walkie-talkie
- jetpack
- radar booster

---

## 7. Sistema de créditos
Necessário:
- saldo atual
- adição de créditos
- remoção de créditos
- validação de saldo

---

## 8. Inventário
Funcionalidades:
- listar itens adquiridos
- capacidade máxima
- quantidades

---

## 9. Navegação
O jogador deve poder:
- selecionar destino
- confirmar viagem
- visualizar local atual

---

## 10. Estado global da sessão
O terminal deve manter:
- créditos
- planeta atual
- inventário
- histórico
- status da nave

Persistente durante execução.

---

# Features intermediárias

## 11. Histórico de comandos
Comandos anteriores.

---

## 12. Logs corporativos
Mensagens estilo:
- ALERT
- WARNING
- TRANSMISSION LOST

---

## 13. Eventos aleatórios
Exemplos:
- tempestade
- interferência
- entidade detectada
- falha de radar

---

## 14. Catálogo de criaturas
Bestiário textual:
- nome
- perigo
- comportamento

---

## 15. Radar
Visualização textual:
- objetos próximos
- ameaças
- sinais

---

## 16. Contratos/quotas
Sistema de:
- meta de créditos
- prazo
- penalidade

---

## 17. Sistema de dias
O tempo avança:
- clima muda
- lojas mudam
- eventos aparecem

---

## 18. Seeds
Gerar mundos/eventos reproduzíveis.

---

# Features avançadas

## 19. Interface TUI
Terminal visual completo:
- painéis
- caixas
- cores
- cursor customizado

---

## 20. Sons do terminal
Beep/efeitos.

---

## 21. Digitação simulada
Texto aparecendo progressivamente.

---

## 22. Arquivos internos
Sistema fake:
- logs
- mensagens
- emails corporativos

---

## 23. Sistema de autenticação
Login:
- guest
- employee
- admin

---

## 24. Rede simulada
Mensagens:
- CONNECTING...
- SIGNAL LOST
- PING

---

## 25. Pseudo filesystem
Comandos:
- ls
- cat
- cd

---

# Comandos importantes

Possíveis comandos:

```text
help
scan
store
buy
inventory
credits
route
travel
status
radar
logs
quota
clear
exit
```

---

# Estados importantes do sistema

Você vai precisar modelar:
- planeta atual
- lista de planetas
- itens
- créditos
- nave
- clima
- eventos
- sessão
- comandos
- erros
- status da corporação

---

# Regras implícitas importantes

## O terminal deve parecer:
- hostil
- corporativo
- industrial
- minimalista
- antigo
- operacional

---

## O texto deve transmitir:
- tensão
- risco
- isolamento
- baixa confiabilidade

---

# Objetivo técnico oculto do projeto

Esse projeto ajuda a aprender:
- modelagem de domínio
- arquitetura de estado
- parsing
- event-driven design
- terminal rendering
- modularização
- error handling
- ownership em Rust
