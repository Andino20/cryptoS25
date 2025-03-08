### Einführung in die Kryptographie - Blatt 01 - Schlager Andreas

#### Aufgabe 2

Die Catmap- und Bakermap-Verschlüsselungen sind chaos-basierte Chiffren, welche ein Bild zuerst unterteilen und anschließend neu organisieren. Um ein Bild mittels Catmap-Verfahren zu verschlüsseln, benötigt man zuerst zwei positive Zahlen $p, q$, die zusammen mit der Anzahl an Iterationen als Schlüssel dienen. In jeder Iteration werden alle Pixel $X = \begin{bmatrix}x&y\end{bmatrix}^T$ des Bilds entsprechend einer Funktion $\Gamma$
$$
\Gamma:\begin{bmatrix}x\\y\end{bmatrix}\to\begin{bmatrix}1&p\\q&pq+1\end{bmatrix}\begin{bmatrix}x\\y\end{bmatrix}
$$
transformiert. Ist die Anzahl der Iterationen zu niedrige, können klare Strukturen innerhalb des verschlüsselten Bildes erkannt werden. Im Falle der Bakermap wird das Bild zuerst horizontal gestreckt, geteilt und übereinander gestapelt, ganz ähnlich zu einem Teig den ein Bäcker zuerst ausrollen und anschließend falten würde. Dafür werden zuerst $k$ Zahlen $n_1,\dots,n_k$ zufällig gewählt, sodass deren Summe gleich der Breite des Bildes ist und jedes Element $n_i$ die Breite ohne Rest teilt. Anschließend wird jeder Pixel, analog zur Catmap, über eine Funktion $B$ an eine neue Position verschoben. 
$$
B(x\quad y)=\begin{pmatrix}q_i\cdot(x-N_i)+(y\mod q_i)&\frac{y-(y\mod q_i)}{q_i+N_i}\end{pmatrix}
$$
Wobei $q_i = N / n_i$ und $N_i = n_1 + \dots + n_i$ ist. Führt man diesen Prozess einige Iteration lang (~ 10 - 45x) aus, erhält man chiffriertes Bild, bei dem die "Schichten" nicht mehr erkenntlich sind. Führt man diese Prozesse ausreichend oft (~ 10 - 45x) durch, erhält man ein chiffriertes Bild, in dem keine Strukturen/"Schichten" erkennbar sind. Ein besonderes Merkmal chaos-basierter Methoden ist die Sensitivität zu den Anfangsbedingungen, d.h. ein leicht abgeändertes Bild führt zu drastisch anderen Ergebnissen.

#### Aufgabe 4

In dem Artikel widerlegen die Autoren zwei zentrale Argumente, die häufig gegen herkömmliche Verschlüsselungsverfahren wie AES und zugunsten chaos-basierter Methoden an Bildern angeführt werden. Darunter die vermeintlich höhere Sicherheit und die angeblich geringere Rechenlast der chaos-basierten Ansätze. Zur Bewertung der Sicherheit chaos-basierter Verschlüsselungen wurden bislang vor allem statistische Tests auf Basis experimenteller Analysen verwendet. Allerdings haben sich viele dieser Verfahren als unsicher erwiesen, da sie in späteren Folge erfolgreich gebrochen werden konnten. Um die Aussagekraft dieser Tests zu hinterfragen, führten die Autoren eine Untersuchung mit bekannten unsicheren Verschlüsselungsmethoden wie der XOR-Verschlüsselung durch. Dabei konnten sie zeigen, dass selbst die unsicheren Verfahren die gängigen Sicherheitsprüfungen bestanden. In einigen Fällen sogar mit besseren Ergebnissen als die chaos-basierten Methoden. Die Autoren konnten außerdem durch ihre Experimente feststellen, dass herkömmliche Verschlüsselungsverfahren wie AES deutlich schneller sind als chaos-basierte Ansätze. Da die AES-Implementierungen kontinuierlich optimiert werden, etwarten die Autoren in diesem Aspekt auch keine Änderung in der Zukunft.