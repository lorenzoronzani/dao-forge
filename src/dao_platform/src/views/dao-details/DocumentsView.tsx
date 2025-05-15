import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Download, Eye, File } from "lucide-react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { Document } from "@/models/entities/Document";
import { formatDate } from "@/utils/date";
import { Button } from "@/components/ui/button";
import { PdfService } from "@/services/pdfService";

interface DocumentsViewProps {
    documents: Document[];
}

export const DocumentsView = ({ documents }: DocumentsViewProps) => {
    return (
        <Card>
            <CardHeader>
                <CardTitle>Documents</CardTitle>
                <CardDescription>Important DAO documents and files</CardDescription>
            </CardHeader>
            <CardContent>
            {documents && documents.length > 0 ? (
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Name</TableHead>
              <TableHead>Owner</TableHead>
              <TableHead>Updated</TableHead>
              <TableHead>Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {documents.map((document) => (
              <TableRow key={document.id}>
                <TableCell className="font-medium">{document.name}</TableCell>
                <TableCell className="font-mono text-xs truncate max-w-[150px]" title={document.owner.toText()}>
                  {document.owner.toText()}
                </TableCell>
                <TableCell>{formatDate(document.updatedAt.getTime())}</TableCell>
                <TableCell>
                  <div className="flex items-center gap-2">
                    <Button variant="ghost" size="sm" onClick={() => PdfService.downloadPdf(document.name, document.content)}>
                      <Download className="h-4 w-4" />
                    </Button>
                    <Button variant="ghost" size="sm" onClick={() => PdfService.openPdf(document.content)}>
                      <Eye className="h-4 w-4" />
                    </Button>
                  </div>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
                ) : (
                    <div className="text-center py-8 text-slate-500">
                        <File className="h-12 w-12 mx-auto mb-4 text-slate-300" />
                        <h3 className="font-medium mb-1">No Documents Yet</h3>
                        <p>There are no documents available for this DAO.</p>
                    </div>
                )}
            </CardContent>
        </Card>
    );
};